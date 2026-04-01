<script lang="ts">
  import Aside from "../components/aside/Aside.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import ShowPorts from "../components/content/ShowPorts.svelte";
  import type { PortInfo, PortFilters } from "../interfaces/Ports";
  import Header from "../components/header/Header.svelte";

  let selectedPort = $state<PortInfo | null>(null);
  let availableStates = $state<string[]>([]);
  let showAdvancedFilters = $state(false);

  // Filter state
  let searchQuery = $state("");
  let protocolFilter = $state<string>("all");
  let stateFilter = $state<string>("all");
  let addressType = $state<string>("all");
  let hideSystemProcesses = $state(false);
  let hideEphemeralPorts = $state(false);
  let portRangeMin = $state<number | null>(null);
  let portRangeMax = $state<number | null>(null);

  // Compute filters object
  let filters = $derived<PortFilters>({
    search_query: searchQuery || null,
    protocol_filter: protocolFilter === "all" ? null : protocolFilter,
    state_filter: stateFilter === "all" ? null : stateFilter,
    address_type: addressType === "all" ? null : addressType,
    hide_system_processes: hideSystemProcesses,
    hide_ephemeral_ports: hideEphemeralPorts,
    port_range_min: portRangeMin,
    port_range_max: portRangeMax,
  });

  function handlePortSelected(port: PortInfo | null) {
    selectedPort = port;
  }

  async function loadAvailableStates() {
    try {
      availableStates = await invoke<string[]>("get_available_states");
    } catch (e) {
      console.error("Failed to load states:", e);
    }
  }

  $effect(() => {
    loadAvailableStates();
  });
</script>

<main
  data-tauri-drag-region
  class="bg-[rgba(31,31,31,0.95)] w-full h-screen flex flex-col p-2"
>
  <!-- HEADER WITH FILTERS -->
  <Header
    bind:searchQuery
    bind:protocolFilter
    bind:stateFilter
    bind:addressType
    bind:hideSystemProcesses
    bind:hideEphemeralPorts
    bind:portRangeMin
    bind:portRangeMax
    bind:showAdvancedFilters
    {availableStates}
  />

  <div class="flex border-2 border-blue-500 w-full h-full">
    <ShowPorts onPortSelected={handlePortSelected} {filters} />

    <Aside selectedPort={selectedPort!} />
  </div>
</main>
