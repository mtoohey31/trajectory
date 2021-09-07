<script lang="ts">
  import { Content } from "carbon-components-svelte";
  import CourseGraph from "./components/CourseGraph.svelte";
  import CourseList from "./components/CourseList.svelte";
  import Header from "./components/Header.svelte";
  import Centered from "./components/Centered.svelte";

  export let userData: Classes.UserData;
  export let syncing: boolean;
  export let firstChange: boolean;

  import type * as Classes from "./classes";

  import { onMount } from "svelte";
  onMount(() => {
    firstChange = true;
  });
</script>

<Header bind:syncing institution={userData.programs[userData.lastProgram].institution} />
<Centered>
  <Content style="height: 100%;">
    <CourseGraph bind:program={userData.programs[userData.lastProgram]} />
    <CourseList bind:program={userData.programs[userData.lastProgram]} />
  </Content>
</Centered>
