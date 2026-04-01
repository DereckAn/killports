<script lang="ts">
  interface Props {
    searchQuery: string;
    protocolFilter: string;
    stateFilter: string;
    addressType: string;
    hideSystemProcesses: boolean;
    hideEphemeralPorts: boolean;
    portRangeMin: number | null;
    portRangeMax: number | null;
    showAdvancedFilters: boolean;
    availableStates: string[];
  }

  let {
    searchQuery = $bindable(),
    protocolFilter = $bindable(),
    stateFilter = $bindable(),
    addressType = $bindable(),
    hideSystemProcesses = $bindable(),
    hideEphemeralPorts = $bindable(),
    portRangeMin = $bindable(),
    portRangeMax = $bindable(),
    showAdvancedFilters = $bindable(),
    availableStates,
  }: Props = $props();
</script>

<!-- HEADER WITH FILTERS -->
<header class="border-2 border-amber-500 bg-amber-400/20 p-3 mb-2">
  <div class="flex items-center justify-between mb-3">
    <h3 class="bg-amber-900 text-white px-3 py-1 text-lg font-bold">
      KillPorts
    </h3>
  </div>

  <!-- Search Bar -->
  <div class="mb-3">
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Search by port, process, or IP address..."
      class="w-full px-4 py-2 bg-gray-800 text-white border border-gray-600 rounded
          focus:outline-none focus:border-emerald-500"
    />
  </div>

  <!-- Quick Filters -->
  <div class="flex gap-3 flex-wrap items-center">
    <select
      bind:value={protocolFilter}
      class="px-3 py-2 bg-gray-800 text-white border border-gray-600 rounded text-sm"
    >
      <option value="all">All Protocols</option>
      <option value="TCP">TCP</option>
      <option value="UDP">UDP</option>
    </select>

    <select
      bind:value={stateFilter}
      class="px-3 py-2 bg-gray-800 text-white border border-gray-600 rounded text-sm"
    >
      <option value="all">All States</option>
      {#each availableStates as state}
        <option value={state}>{state}</option>
      {/each}
    </select>

    <select
      bind:value={addressType}
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
        class="w-4 h-4"
      />
      Hide System
    </label>

    <label class="flex items-center gap-2 text-white text-sm cursor-pointer">
      <input
        type="checkbox"
        bind:checked={hideEphemeralPorts}
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
    <div class="mt-3 p-3 bg-gray-800 rounded border border-gray-600">
      <h3 class="text-white font-semibold mb-2 text-sm">Port Range Filter</h3>
      <div class="flex gap-3 items-center">
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
      </div>
    </div>
  {/if}
</header>
