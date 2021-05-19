<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import Dashboard from "./Dasbhoard.svelte";
  import Login from "./Login.svelte";
  import CreateAccount from "./CreateAccount.svelte";
  import type { UserData } from "./classes";

  export let url = "";
  let userData: UserData;
  let username: string;
  let hashedPasswd: CryptoKey;
  let vaultSalt: string;
</script>

<Router {url}>
  <Route path="/">
    <Dashboard bind:userData bind:username bind:hashedPasswd bind:vaultSalt />
  </Route>
  <Route path="login">
    <Login bind:userData bind:username bind:hashedPasswd bind:vaultSalt />
  </Route>
  <Route path="login/create">
    <CreateAccount />
  </Route>
</Router>
