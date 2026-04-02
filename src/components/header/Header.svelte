<script lang="ts">
  interface Props {
    searchQuery: string;
    protocolFilter: string[];
    stateFilter: string[];
    addressType: string[];
    hideSystemProcesses: boolean;
    hideEphemeralPorts: boolean;
    portRangeMin: number | null;
    portRangeMax: number | null;
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
    availableStates,
  }: Props = $props();

  let showPopup = $state(false);

  function toggleFilter(arr: string[], value: string): string[] {
    return arr.includes(value)
      ? arr.filter((v) => v !== value)
      : [...arr, value];
  }

  function resetFilters() {
    protocolFilter = [];
    stateFilter = [];
    addressType = [];
    hideSystemProcesses = false;
    hideEphemeralPorts = false;
    portRangeMin = null;
    portRangeMax = null;
  }

  let activeFilterCount = $derived(
    protocolFilter.length +
      stateFilter.length +
      addressType.length +
      (hideSystemProcesses ? 1 : 0) +
      (hideEphemeralPorts ? 1 : 0) +
      (portRangeMin !== null ? 1 : 0) +
      (portRangeMax !== null ? 1 : 0),
  );
</script>

<!-- HEADER WITH FILTERS -->
<header data-tauri-drag-region class="border-2 border-amber-500 bg-amber-400/20 p-3 mb-2 flex justify-between">
  <div class="flex items-center justify-between mb-3">
    <h3 class="bg-amber-900 text-white px-3 py-1 text-lg font-bold">
      KillPorts
    </h3>
  </div>

  <div class="flex gap-2 w-1/2">
    <!-- Filters button with popup -->
    <div class="relative ">
      <button
        onclick={() => (showPopup = !showPopup)}
        class="px-3 py-2 bg-gray-700 hover:bg-gray-600 text-white text-sm rounded flex items-center gap-2"
      >
        Filters
        {#if activeFilterCount > 0}
          <span
            class="bg-amber-500 text-black rounded-full w-5 h-5 text-xs flex items-center justify-center font-bold"
          >
            {activeFilterCount}
          </span>
        {/if}
      </button>

      {#if showPopup}
        <!-- Backdrop -->
        <div
          class="fixed inset-0 z-40"
          role="presentation"
          onclick={() => (showPopup = false)}
        ></div>

        <!-- Popup panel -->
        <div
          class="absolute top-full left-0 mt-1 bg-gray-900 border border-gray-600 rounded shadow-xl z-50 w-72 p-4"
        >
          <div class="flex justify-between items-center mb-3">
            <span class="text-white font-semibold text-sm">Filters</span>
            <div class="flex gap-2">
              {#if activeFilterCount > 0}
                <button
                  onclick={resetFilters}
                  class="text-amber-400 hover:text-amber-300 text-xs"
                >
                  Reset all
                </button>
              {/if}
              <button
                onclick={() => (showPopup = false)}
                class="text-gray-400 hover:text-white"
              >
                ✕
              </button>
            </div>
          </div>

          <!-- Protocol -->
          <div class="mb-4">
            <p class="text-gray-400 text-xs uppercase tracking-wide mb-2">
              Protocol
            </p>
            <div class="flex flex-col gap-1">
              {#each ["TCP", "UDP"] as proto}
                <label
                  class="flex items-center gap-2 text-white text-sm cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={protocolFilter.includes(proto)}
                    onchange={() =>
                      (protocolFilter = toggleFilter(protocolFilter, proto))}
                    class="w-4 h-4 accent-amber-500"
                  />
                  {proto}
                </label>
              {/each}
            </div>
          </div>

          <!-- State -->
          <div class="mb-4">
            <p class="text-gray-400 text-xs uppercase tracking-wide mb-2">
              State
            </p>
            <div class="flex flex-col gap-1 max-h-40 overflow-y-auto">
              {#each availableStates as state}
                <label
                  class="flex items-center gap-2 text-white text-sm cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={stateFilter.includes(state)}
                    onchange={() =>
                      (stateFilter = toggleFilter(stateFilter, state))}
                    class="w-4 h-4 accent-amber-500"
                  />
                  {state}
                </label>
              {/each}
            </div>
          </div>

          <!-- Address Type -->
          <div class="mb-4">
            <p class="text-gray-400 text-xs uppercase tracking-wide mb-2">
              Address Type
            </p>
            <div class="flex flex-col gap-1">
              {#each [{ value: "localhost", label: "Localhost Only" }, { value: "network", label: "Network Only" }] as opt}
                <label
                  class="flex items-center gap-2 text-white text-sm cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={addressType.includes(opt.value)}
                    onchange={() =>
                      (addressType = toggleFilter(addressType, opt.value))}
                    class="w-4 h-4 accent-amber-500"
                  />
                  {opt.label}
                </label>
              {/each}
            </div>
          </div>

          <!-- Options -->
          <div class="mb-4">
            <p class="text-gray-400 text-xs uppercase tracking-wide mb-2">
              Options
            </p>
            <div class="flex flex-col gap-1">
              <label
                class="flex items-center gap-2 text-white text-sm cursor-pointer"
              >
                <input
                  type="checkbox"
                  bind:checked={hideSystemProcesses}
                  class="w-4 h-4 accent-amber-500"
                />
                Hide System Processes
              </label>
              <label
                class="flex items-center gap-2 text-white text-sm cursor-pointer"
              >
                <input
                  type="checkbox"
                  bind:checked={hideEphemeralPorts}
                  class="w-4 h-4 accent-amber-500"
                />
                Hide Ephemeral Ports (49152+)
              </label>
            </div>
          </div>

          <!-- Port Range -->
          <div>
            <p class="text-gray-400 text-xs uppercase tracking-wide mb-2">
              Port Range
            </p>
            <div class="flex gap-3 items-center">
              <div class="flex items-center gap-1">
                <span class="text-gray-400 text-xs">Min:</span>
                <input
                  type="number"
                  bind:value={portRangeMin}
                  placeholder="1"
                  min="1"
                  max="65535"
                  class="w-20 px-2 py-1 bg-gray-700 text-white border border-gray-600 rounded text-sm"
                />
              </div>
              <div class="flex items-center gap-1">
                <span class="text-gray-400 text-xs">Max:</span>
                <input
                  type="number"
                  bind:value={portRangeMax}
                  placeholder="65535"
                  min="1"
                  max="65535"
                  class="w-20 px-2 py-1 bg-gray-700 text-white border border-gray-600 rounded text-sm"
                />
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Search Bar -->
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Search by port, process, or IP address..."
      class="flex-1 px-4 bg-gray-800 text-white border border-gray-600 rounded max-w-125
        focus:outline-none focus:border-emerald-500"
    />
  </div>
</header>
