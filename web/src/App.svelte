<script lang="ts">
  import { navigate, Router, Route } from "svelte-routing";
  import Dashboard from "./Dasbhoard.svelte";
  import Login from "./Login.svelte";
  import CreateAccount from "./CreateAccount.svelte";
  import Account from "./Account.svelte";
  import Theme from "./components/Theme.svelte";
  import Import from "./Import.svelte";
  import type { UserData } from "./classes";

  import { globalHistory } from "svelte-routing/src/history";
  globalHistory.listen((h) => {
    firstChange = true;
  });

  import { onMount } from "svelte";
  onMount(() => {
    if (location.pathname !== "/login" && !userData) {
      navigate("/login");
    }
  });

  export let url = "";
  let userData: UserData;
  let username: string;
  let loginKey: CryptoKey;
  let vaultKey: CryptoKey;

  async function updateData() {
    const exportedKey = await window.crypto.subtle.exportKey("raw", loginKey);
    const exportedKeyBuffer = new Uint8Array(exportedKey);
    let dec = new TextDecoder();
    let auth =
      "Basic " +
      window.btoa(
        unescape(
          encodeURIComponent(`${username}:${dec.decode(exportedKeyBuffer)}`)
        )
      );
    let enc = new TextEncoder();
    let iv = window.crypto.getRandomValues(new Uint8Array(16));
    let encryptedUserData = new Uint8Array(
      await window.crypto.subtle.encrypt(
        {
          name: "AES-GCM",
          iv: iv,
        },
        vaultKey,
        enc.encode(JSON.stringify(userData))
      )
    );
    const options = {
      method: "PUT",
      body: new Uint8Array([...iv, ...encryptedUserData]),
      headers: {
        Authorization: auth,
      },
    };
    fetch("/api/users/data", options).then(async (res) => {
      if (res.status !== 200) {
        // TODO: Handle errors here
      }
    });
  }

  let firstChange = true;
  let syncing = false;

  async function sync(_userData: UserData) {
    if (firstChange) {
      firstChange = false;
    } else {
      syncing = true;
    }
  }

  setInterval(async () => {
    if (syncing) {
      updateData().then(() => {
        syncing = false;
      });
    }
  }, 10000);

  $: theme = userData ? userData.settings.theme : "g10";
  $: sync(userData);
  $: window.onbeforeunload = syncing
    ? (e: Event) => {
        e.preventDefault();
        return e;
      }
    : undefined;
</script>

<Theme persist bind:theme>
  <Router {url}>
    <Route path="/">
      {#if userData}
        <Dashboard bind:userData bind:syncing bind:firstChange />
      {/if}
    </Route>
    <Route path="login">
      <Login bind:userData bind:username bind:loginKey bind:vaultKey />
    </Route>
    <Route path="login/create">
      <CreateAccount />
    </Route>
    <Route path="account">
      {#if userData}
        <Account bind:userData bind:username bind:loginKey />
      {/if}
    </Route>
    <Route path="account/import">
      <Import bind:userData />
    </Route>
  </Router>
</Theme>
