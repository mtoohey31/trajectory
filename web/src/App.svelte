<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import Dashboard from "./Dasbhoard.svelte";
  import Login from "./Login.svelte";
  import CreateAccount from "./CreateAccount.svelte";
  import type { UserData } from "./classes";

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
</script>

<Router {url}>
  <Route path="/">
    <Dashboard
      bind:userData
      bind:username
      bind:loginKey
      bind:vaultKey
      {updateData}
    />
  </Route>
  <Route path="login">
    <Login bind:userData bind:username bind:loginKey bind:vaultKey />
  </Route>
  <Route path="login/create">
    <CreateAccount />
  </Route>
</Router>
