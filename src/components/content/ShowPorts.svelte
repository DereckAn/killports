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
  }

  let ports = $state<PortInfo[]>([]);
  let loading = $state(false);
  let error = $state("");

  async function fetchPorts() {
    loading = true;
    error = "";
    try {
      ports = await invoke<PortInfo[]>("get_active_ports");
    } catch (e) {
      error = `Failed to fetch ports: ${e}`;
    } finally {
      loading = false;
    }
  }
</script>

<section class="flex-1 min-w-0 border-2 border-green-500">
  <button onclick={fetchPorts} class="px-4 py-2 bg-blue-500 text-white rounded">
    Get active ports
  </button>
  {#if loading}
    <p class="text-yellow-500 mt-2">Loading...</p>
  {:else if error}
    <p class="text-red-500 mt-2">{error}</p>
  {/if}
  {#if ports.length > 0}
    <div class="overflow-auto max-h-96 border-4 border-purple-500 w-full">
      <table class="text-white w-full table-fixed">
        <thead>
          <tr class="bg-gray-700">
            <th class="w-[10%]">Protocol</th>
            <th class="w-[20%]">Local Address</th>
            <th class="w-[10%]">Local Port</th>
            <th class="w-[20%]">Remote Address</th>
            <th class="w-[10%]">Remote Port</th>
            <th class="w-[10%]">State</th>
            <th class="w-[10%]">Process</th>
          </tr>
        </thead>
        <tbody>
          {#each ports as port}
            <tr class="border-b border-gray-600">
              <td>{port.protocol}</td>
              <td class="w-24 max-w-24 truncate">{port.local_address}</td>
              <td>{port.local_port}</td>
              <td>{port.remote_address}</td>
              <td>{port.remote_port}</td>
              <td>{port.state}</td>
              <td>{port.process_name || "N/A"}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</section>
