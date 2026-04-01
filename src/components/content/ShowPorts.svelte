<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { PortFilters, PortInfo } from "../../interfaces/Ports";
  import eyeIcon from "../../assets/eye.svg?raw";
  import killIcon from "../../assets/kill.svg?raw";

  interface Props {
    onPortSelected: (port: PortInfo | null) => void;
    filters: PortFilters;
  }

  let { filters, onPortSelected }: Props = $props();
  let selectedPort = $state<PortInfo | null>(null);
  let ports = $state<PortInfo[]>([]);
  let loading = $state(false);
  let error = $state("");

  async function fetchPOrts() {
    loading = true;
    error = "";
    try {
      ports = await invoke<PortInfo[]>("get_active_ports", { filters });
    } catch (e) {
      error = "Failed to load ports. Please try again.";
    } finally {
      loading = false;
    }
  }

  function selectPort(port: PortInfo) {
    selectedPort = port;
    onPortSelected(port);
  }

  // Fetch ports whenever filters change
  $effect(() => {
    fetchPOrts();
  });
</script>

<section
  class="flex-1 border-2 border-green-500 flex flex-col overflow-hidden p-4"
>
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
            <th class="w-[10%]">State</th>
            <th class="w-[10%]">Process</th>
            <th class="w-[10%]">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each ports as port}
            <tr
              class="border-b border-gray-700 hover:bg-gray-800 cursor-pointer transition-colors
  duration-300 {selectedPort?.local_port === port.local_port &&
              selectedPort?.pid === port.pid
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
