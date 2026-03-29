<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  interface PortInfo {
    local_port: number;
    local_address: string;
    remote_port: number;
    remote_address: string;
    state: string;
    protocol: string;
    process_name: string | null;
    pid: number | null;
  }

  interface PortFilters {
    search_query: string | null;
    protocol_filter: string | null;
    state_filter: string | null;
    address_type: string | null;
    hide_system_processes: boolean;
    hide_ephemeral_ports: boolean;
    port_range_min: number | null;
    port_range_max: number | null;
  }
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

  // Preset filters
  function applyPreset(preset: string) {
    switch (preset) {
      case "clean":
        // Clean View: Only listening ports, hide ephemeral, hide system
        stateFilter = "Listen";
        hideEphemeralPorts = true;
        hideSystemProcesses = true;
        addressType = "all";
        protocolFilter = "all";
        searchQuery = "";
        break;
      case "security":
        // Security Audit: Only listening, network-accessible
        stateFilter = "Listen";
        addressType = "network";
        hideSystemProcesses = false;
        hideEphemeralPorts = true;
        protocolFilter = "all";
        searchQuery = "";
        break;
      case "developer":
        // Developer View: Localhost only, listening
        stateFilter = "Listen";
        addressType = "localhost";
        hideSystemProcesses = true;
        hideEphemeralPorts = true;
        protocolFilter = "all";
        searchQuery = "";
        break;
      case "active":
        // Active Connections: Established, no localhost
        stateFilter = "Established";
        addressType = "network";
        hideSystemProcesses = false;
        hideEphemeralPorts = false;
        protocolFilter = "all";
        searchQuery = "";
        break;
      case "all":
        // Reset all filters
        resetFilters();
        break;
    }
    fetchPorts();
  }

  function resetFilters() {
    searchQuery = "";
    protocolFilter = "all";
    stateFilter = "all";
    addressType = "all";
    hideSystemProcesses = false;
    hideEphemeralPorts = false;
    portRangeMin = null;
    portRangeMax = null;
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
  <h1 class="text-2xl font-bold mb-4 text-emerald-400">Port Monitor</h1>

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

  <!-- Preset Filter Buttons -->
  <div class="flex gap-2 mb-4 flex-wrap">
    <button
      onclick={() => applyPreset("clean")}
      class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded"
    >
      🧹 Clean View
    </button>
    <button
      onclick={() => applyPreset("security")}
      class="px-3 py-1 bg-red-600 hover:bg-red-700 text-white text-sm rounded"
    >
      🔒 Security Audit
    </button>
    <button
      onclick={() => applyPreset("developer")}
      class="px-3 py-1 bg-purple-600 hover:bg-purple-700 text-white text-sm rounded"
    >
      💻 Developer
    </button>
    <button
      onclick={() => applyPreset("active")}
      class="px-3 py-1 bg-green-600 hover:bg-green-700 text-white text-sm rounded"
    >
      🌐 Active Connections
    </button>
    <button
      onclick={() => applyPreset("all")}
      class="px-3 py-1 bg-gray-600 hover:bg-gray-700 text-white text-sm rounded"
    >
      ⭕ Show All
    </button>
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
      <table class="w-full text-white text-sm">
        <thead class="sticky top-0 bg-gray-800">
          <tr class="border-b border-gray-600">
            <th class="p-2 text-left">Protocol</th>
            <th class="p-2 text-left">Local Address</th>
            <th class="p-2 text-left">Local Port</th>
            <th class="p-2 text-left">Remote Address</th>
            <th class="p-2 text-left">Remote Port</th>
            <th class="p-2 text-left">State</th>
            <th class="p-2 text-left">Process</th>
          </tr>
        </thead>
        <tbody>
          {#each ports as port}
            <tr class="border-b border-gray-700 hover:bg-gray-800">
              <td class="p-2">
                <span
                  class="px-2 py-1 rounded text-xs {port.protocol === 'TCP'
                    ? 'bg-blue-900 text-blue-200'
                    : 'bg-purple-900 text-purple-200'}"
                >
                  {port.protocol}
                </span>
              </td>
              <td class="p-2 font-mono text-xs">{port.local_address}</td>
              <td class="p-2 font-mono font-bold text-emerald-400">
                {port.local_port}
              </td>
              <td class="p-2 font-mono text-xs">{port.remote_address}</td>
              <td class="p-2 font-mono text-xs">{port.remote_port || "-"}</td>
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
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</section>
