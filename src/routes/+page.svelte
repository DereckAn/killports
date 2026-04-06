<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Aside from "../components/aside/Aside.svelte";
  import ShowPorts from "../components/content/ShowPorts.svelte";
  import Header from "../components/header/Header.svelte";
  import type { PortFilters, PortInfo } from "../interfaces/Ports";

  let selectedPort = $state<PortInfo | null>(null);
  let availableStates = $state<string[]>([]);
  let watchedPorts = $state<Set<number>>(new Set()); // A set is like an array but with no duplicates

  // Filter state
  let searchQuery = $state("");
  let protocolFilter = $state<string[]>([]);
  let stateFilter = $state<string[]>([]);
  let addressType = $state<string[]>([]);
  let hideSystemProcesses = $state(false);
  let hideEphemeralPorts = $state(false);
  let portRangeMin = $state<number | null>(null);
  let portRangeMax = $state<number | null>(null);

  // Compute filters object
  let filters = $derived<PortFilters>({
    search_query: searchQuery || null,
    protocol_filter: protocolFilter.length > 0 ? protocolFilter : null,
    state_filter: stateFilter.length > 0 ? stateFilter : null,
    address_type: addressType.length > 0 ? addressType : null,
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

  async function toggleWatch(port: PortInfo) {
    if (!port.command_line) return;

    const isWatched = watchedPorts.has(port.local_port);

    try {
      if (isWatched) {
        await invoke("unwatch_port", { port: port.local_port });
        watchedPorts.delete(port.local_port);
        watchedPorts = new Set(watchedPorts); // Trigger reactivity
      } else {
        await invoke("watch_port", {
          port: port.local_port,
          commandLine: port.command_line,
        });
        watchedPorts.add(port.local_port);
        watchedPorts = new Set(watchedPorts); // Trigger reactivity
      }
    } catch (e) {
      console.error(`Failed to ${isWatched ? "unwatch" : "watch"} port:`, e);
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
    {availableStates}
  />

  <div class="flex border-2 border-blue-500 w-full h-full">
    <ShowPorts onPortSelected={handlePortSelected} {filters} {watchedPorts} />

    <Aside
      selectedPort={selectedPort!}
      {watchedPorts}
      onToggleWatch={toggleWatch}
    />
  </div>
</main>
