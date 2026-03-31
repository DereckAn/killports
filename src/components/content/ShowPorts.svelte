<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { PortFilters, PortInfo } from "../../interfaces/Ports";
  import eyeIcon from "../../assets/eye.svg?raw";
  import killIcon from "../../assets/kill.svg?raw";

  interface Props {
    onPortSelected: (port: PortInfo | null) => void;
  }

  let { onPortSelected }: Props = $props();
  let selectedPort = $state<PortInfo | null>(null);
  let ports = $state<PortInfo[]>([]);
  let loading = $state(false);
  let error = $state("");
  let availableStates = $state<string[]>([]);

  // Filter state
  let searchQuery = $state("");
  let protocolFilter = $state<string>("all");
  let stateFilter = $state<string>("all");
  let addressType = $state<string>("all");
  let hideSystemProcesses = $state(false);
  let hideEphemeralPorts = $state(false);
  let portRangeMin = $state<number | null>(null);
  let portRangeMax = $state<number | null>(null);

  // UI state
  let showAdvancedFilters = $state(false);

  async function fetchPorts() {
    loading = true;
    error = "";
    try {
      const filters: PortFilters = {
        search_query: searchQuery || null,
        protocol_filter: protocolFilter === "all" ? null : protocolFilter,
        state_filter: stateFilter === "all" ? null : stateFilter,
        address_type: addressType === "all" ? null : addressType,
        hide_system_processes: hideSystemProcesses,
        hide_ephemeral_ports: hideEphemeralPorts,
        port_range_min: portRangeMin,
        port_range_max: portRangeMax,
      };

      ports = await invoke<PortInfo[]>("get_active_ports", { filters });
    } catch (e) {
      error = `Failed to fetch ports: ${e}`;
    } finally {
      loading = false;
    }
  }

  async function loadAvailableStates() {
    try {
      availableStates = await invoke<string[]>("get_available_states");
    } catch (e) {
      console.error("Failed to load states:", e);
    }
  }

  function selectPort(port: PortInfo) {
    selectedPort = port;
    onPortSelected(port);
  }

  function clearSelection() {
    selectedPort = null;
    onPortSelected(null);
  }

  // Load states on mount
  $effect(() => {
    loadAvailableStates();
    fetchPorts();
  });
</script>

<section
  class="flex-1 border-2 border-green-500 flex flex-col overflow-hidden p-4"
>
  <!-- Search Bar -->
  <div class="mb-4">
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Search by port, process, or IP address..."
      class="w-full px-4 py-2 bg-gray-800 text-white border border-gray-600 rounded
  focus:outline-none focus:border-emerald-500"
      onkeyup={(e) => e.key === "Enter" && fetchPorts()}
    />
  </div>

  <!-- Quick Filters -->
  <div class="flex gap-4 mb-4 flex-wrap items-center">
    <select
      bind:value={protocolFilter}
      onchange={fetchPorts}
      class="px-3 py-2 bg-gray-800 text-white border border-gray-600 rounded text-sm"
    >
      <option value="all">All Protocols</option>
      <option value="TCP">TCP</option>
      <option value="UDP">UDP</option>
    </select>

    <select
      bind:value={stateFilter}
      onchange={fetchPorts}
      class="px-3 py-2 bg-gray-800 text-white border border-gray-600 rounded text-sm"
    >
      <option value="all">All States</option>
      {#each availableStates as state}
        <option value={state}>{state}</option>
      {/each}
    </select>

    <select
      bind:value={addressType}
      onchange={fetchPorts}
      class="px-3 py-2 bg-gray-800 text-white border border-gray-600 rounded text-sm"
    >
      <option value="all">All Addresses</option>
      <option value="localhost">Localhost Only</option>
      <option value="network">Network Only</option>
    </select>

    <label class="flex items-center gap-2 text-white text-sm cursor-pointer">
      <input
        type="checkbox"
        bind:checked={hideSystemProcesses}
        onchange={fetchPorts}
        class="w-4 h-4"
      />
      Hide System
    </label>

    <label class="flex items-center gap-2 text-white text-sm cursor-pointer">
      <input
        type="checkbox"
        bind:checked={hideEphemeralPorts}
        onchange={fetchPorts}
        class="w-4 h-4"
      />
      Hide Ephemeral
    </label>

    <button
      onclick={() => (showAdvancedFilters = !showAdvancedFilters)}
      class="px-3 py-2 bg-gray-700 hover:bg-gray-600 text-white text-sm rounded"
    >
      {showAdvancedFilters ? "▲ Advanced" : "▼ Advanced"}
    </button>
  </div>

  <!-- Advanced Filters (Collapsible) -->
  {#if showAdvancedFilters}
    <div class="mb-4 p-4 bg-gray-800 rounded border border-gray-600">
      <h3 class="text-white font-semibold mb-3">Port Range Filter</h3>
      <div class="flex gap-4 items-center">
        <div class="flex items-center gap-2">
          <label class="text-white text-sm">Min:</label>
          <input
            type="number"
            bind:value={portRangeMin}
            placeholder="1"
            min="1"
            max="65535"
            class="w-24 px-2 py-1 bg-gray-700 text-white border border-gray-600 rounded text-sm"
          />
        </div>
        <div class="flex items-center gap-2">
          <label class="text-white text-sm">Max:</label>
          <input
            type="number"
            bind:value={portRangeMax}
            placeholder="65535"
            min="1"
            max="65535"
            class="w-24 px-2 py-1 bg-gray-700 text-white border border-gray-600 rounded text-sm"
          />
        </div>
        <button
          onclick={fetchPorts}
          class="px-4 py-1 bg-emerald-600 hover:bg-emerald-700 text-white text-sm rounded"
        >
          Apply Range
        </button>
      </div>
    </div>
  {/if}

  <!-- Results Summary -->
  <div class="mb-2 text-gray-400 text-sm">
    {#if loading}
      <span class="text-yellow-500">Loading...</span>
    {:else if ports.length > 0}
      Showing {ports.length} port{ports.length !== 1 ? "s" : ""}
    {:else if error}
      <span class="text-red-500">{error}</span>
    {:else}
      No ports found with current filters
    {/if}
  </div>

  <!-- Ports Table -->
  <div class="flex-1 overflow-auto">
    {#if ports.length > 0}
      <table class="w-full text-white text-sm table-fixed">
        <thead class="sticky top-0 bg-gray-800">
          <tr class="border-b border-gray-600">
            <th class="w-[10%]">Port</th>
            <th class="w-[10%]">Address</th>
            <th class="w-[10%]">PID</th>
            <!-- <th class="w-[10%]">Remote Address</th>
            <th class="w-[10%]">Remote Port</th> -->
            <th class="w-[10%]">State</th>
            <th class="w-[10%]">Process</th>
            <th class="w-[10%]">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each ports as port}
            <tr
              class="border-b border-gray-700 hover:bg-gray-800 cursor-pointer transition-colors duration-300 {selectedPort?.local_port ===
                port.local_port && selectedPort?.pid === port.pid
                ? 'bg-emerald-900/30 border-l-4 border-l-emerald-500'
                : ''}"
              onclick={() => selectPort(port)}
              role="button"
              tabindex="0"
            >
              <td class="p-2 font-mono font-bold text-emerald-400">
                {port.local_port}
              </td>
              <td class="p-2 font-mono text-xs">{port.local_address}</td>
              <td class="p-2 font-mono text-xs">{port.pid}</td>
              <!-- <td class="p-2 font-mono text-xs">{port.remote_address}</td>
              <td class="p-2 font-mono text-xs">{port.remote_port || "-"}</td> -->
              <td class="p-2">
                <span
                  class="px-2 py-1 rounded text-xs {port.state === 'Listen'
                    ? 'bg-green-900 text-green-200'
                    : port.state === 'Established'
                      ? 'bg-blue-900 text-blue-200'
                      : port.state === 'TimeWait'
                        ? 'bg-yellow-900 text-yellow-200'
                        : 'bg-gray-700 text-gray-300'}"
                >
                  {port.state}
                </span>
              </td>
              <td
                class="p-2 truncate max-w-xs"
                title={port.process_name || "N/A"}
              >
                {port.process_name || "N/A"}
              </td>
              <td class="p-2">
                <div class="flex gap-2 items-center">
                  <button
                    onclick={(e) => {
                      e.stopPropagation();
                      selectPort(port);
                    }}
                    class="text-blue-400 hover:text-blue-200 transition-colors"
                    title="View"
                  >
                    {@html eyeIcon}
                  </button>
                  <button
                    onclick={(e) => {
                      e.stopPropagation();
                    }}
                    class="text-red-400 hover:text-red-200 transition-colors"
                    title="Kill"
                  >
                    {@html killIcon}
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</section>
