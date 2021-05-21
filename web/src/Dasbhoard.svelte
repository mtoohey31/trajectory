<script lang="ts">
  import { Content } from "carbon-components-svelte";
  import CourseGraph from "./components/CourseGraph.svelte";
  import CourseList from "./components/CourseList.svelte";
  import Header from "./components/Header.svelte";

  export let userData: Classes.UserData;
  export let username: string;
  export let loginKey: CryptoKey;
  export let syncing: boolean;

  import type * as Classes from "./classes";
</script>

{#if userData}
  <Header bind:userData bind:username bind:loginKey bind:syncing />
  <main>
    <Content style="height: 100%;">
      <CourseGraph bind:program={userData.programs[0]} />
      <CourseList bind:program={userData.programs[0]} />
    </Content>
  </main>
{/if}

<style>
  main {
    padding-top: 5rem;
    width: clamp(45ch, calc(100vw - 4rem), 100ch);
    background-color: var(--theme-background-content);
    margin: 0 auto;
    border-radius: 1rem;
  }
</style>
