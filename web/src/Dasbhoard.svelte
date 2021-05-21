<script lang="ts">
  import { Content } from "carbon-components-svelte";
  import { navigate } from "svelte-routing";
  import CourseGraph from "./components/CourseGraph.svelte";
  import CourseList from "./components/CourseList.svelte";
  import Theme from "./components/Theme.svelte";
  import Header from "./components/Header.svelte";

  let theme = "g10";

  export let userData: Classes.UserData;
  export let username: string;
  export let loginKey: CryptoKey;
  export let vaultKey: CryptoKey;
  export let updateData: () => void;

  import { onMount } from "svelte";
  onMount(() => {
    if (!userData) {
      navigate("/login");
    }
  });

  import type * as Classes from "./classes";
</script>

<Theme persist bind:theme>
  {#if userData}
    <Header bind:theme bind:userData bind:username bind:loginKey {updateData} />
    <main>
      <Content style="height: 100%;">
        <CourseGraph bind:program={userData.programs[0]} />
        <CourseList bind:program={userData.programs[0]} />
      </Content>
    </main>
  {/if}
</Theme>

<style>
  main {
    padding-top: 5rem;
    width: clamp(45ch, calc(100vw - 4rem), 100ch);
    background-color: var(--theme-background-content);
    margin: 0 auto;
    border-radius: 1rem;
  }
</style>
