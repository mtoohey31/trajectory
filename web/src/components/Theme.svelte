<script context="module" lang="ts">
  export const themes: Array<theme> = ["white", "g10", "g90", "g100"];
  export const themeNames = ["White", "Gray 10", "Gray 90", "Gray 100"];
</script>

<script content="module" lang="ts">
  export let persist = false;
  export let persistKey = "theme";
  export let theme = "white";
  import { onMount, afterUpdate, setContext } from "svelte";
  import { writable, derived } from "svelte/store";
  import type { theme } from "../classes";
  const isValidTheme = (value) => themes.includes(value);
  const isDark = (value) =>
    isValidTheme(value) && (value === "g90" || value === "g100");
  const dark = writable(isDark(theme));
  const light = derived(dark, (_) => !_);
  setContext("Theme", {
    updateVar: (name, value) => {
      document.documentElement.style.setProperty(name, value);
    },
    dark,
    light,
  });
  onMount(() => {
    try {
      const persisted_theme = localStorage.getItem(persistKey);
      if (isValidTheme(persisted_theme)) {
        theme = persisted_theme;
      }
    } catch (error) {
      console.error(error);
    }
  });
  afterUpdate(() => {
    if (isValidTheme(theme)) {
      document.documentElement.setAttribute("theme", theme);
      if (persist) {
        localStorage.setItem(persistKey, theme);
      }
    } else {
      console.warn(
        `"${theme}" is not a valid Carbon theme. Choose from available themes: ${JSON.stringify(
          themes
        )}`
      );
    }
  });
  $: dark.set(isDark(theme));
</script>

<slot />
