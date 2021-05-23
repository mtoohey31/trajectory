<script lang="ts">
  import {
    Button,
    ButtonSet,
    Content,
    Grid,
    Row,
    Column,
    TextInput,
    PasswordInput,
  } from "carbon-components-svelte";
  import Login20 from "carbon-icons-svelte/lib/Login20/Login20.svelte";
  import Add20 from "carbon-icons-svelte/lib/Add20/Add20.svelte";
  import { navigate } from "svelte-routing";
  import * as Classes from "./classes";

  export let userData: Classes.UserData;
  export let username: string;
  export let loginKey: CryptoKey;
  export let vaultKey: CryptoKey;
  export let firstChange: boolean;

  async function login() {
    let strUtf8 = unescape(encodeURIComponent(enteredUsername));
    let salt = new Uint8Array(strUtf8.length);
    for (let i = 0; i < strUtf8.length; i++) {
      salt[i] = strUtf8.charCodeAt(i);
    }
    let enc = new TextEncoder();
    let keyMaterial = await window.crypto.subtle.importKey(
      "raw",
      enc.encode(password),
      { name: "PBKDF2" },
      false,
      ["deriveKey"]
    );
    let derivedKey = await window.crypto.subtle.deriveKey(
      { name: "PBKDF2", salt: salt, iterations: 100000, hash: "SHA-256" },
      keyMaterial,
      { name: "AES-GCM", length: 256 },
      true,
      ["encrypt"]
    );
    let exportedKey = await window.crypto.subtle.exportKey("raw", derivedKey);
    let exportedKeyBuffer = new Uint8Array(exportedKey);
    let dec = new TextDecoder();
    let auth =
      "Basic " +
      window.btoa(
        unescape(
          encodeURIComponent(
            `${enteredUsername}:${dec.decode(exportedKeyBuffer)}`
          )
        )
      );
    const options = {
      method: "GET",
      headers: {
        Authorization: auth,
        "Content-Type": "application/json",
      },
    };
    fetch("/api/users/data", options).then(async (res) => {
      if (res.status === 200) {
        invalidText = "";
        username = enteredUsername;
        loginKey = derivedKey;
        let resData = (await res.body.getReader().read()).value;
        vaultKey = await window.crypto.subtle.deriveKey(
          {
            name: "PBKDF2",
            salt: resData.slice(0, 16),
            iterations: 100000,
            hash: "SHA-256",
          },
          keyMaterial,
          { name: "AES-GCM", length: 256 },
          true,
          ["encrypt", "decrypt"]
        );
        if (resData.length > 16) {
          let decryptedData = await window.crypto.subtle.decrypt(
            {
              name: "AES-GCM",
              iv: resData.slice(16, 32),
            },
            vaultKey,
            resData.slice(32)
          );
          userData = Classes.UserData.from(
            JSON.parse(dec.decode(decryptedData))
          );
        } else {
          userData = new Classes.UserData(
            [new Classes.Program("", [], new Classes.ProgramSettings())],
            new Classes.UserSettings("g10")
          );
        }
        navigate("./");
      } else if (res.status === 403) {
        invalidText = "Incorrect username or password";
      } else {
        invalidText = "An unknown error occurred";
      }
    });
  }

  let enteredUsername = "";
  let password = "";
  let enteringPassword = false;
  let invalidText = "";
  $: invalid = !!invalidText;
</script>

<svelte:window
  on:keydown={(event) => {
    if (enteringPassword && event.key == "Enter") {
      login();
    }
  }}
/>

<Content>
  <Grid>
    <Row>
      <Column>
        <h1>Login</h1>
        <TextInput
          autofocus
          labelText="Username"
          bind:value={enteredUsername}
          bind:invalid
        />
        <PasswordInput
          labelText="Password"
          bind:value={password}
          bind:invalid
          bind:invalidText
          on:focus={() => (enteringPassword = true)}
          on:blur={() => (enteringPassword = false)}
        />
        <ButtonSet style="margin-top: 1rem;">
          <Button icon={Login20} on:click={login}>Login</Button>
          <Button
            icon={Add20}
            kind="tertiary"
            on:click={() => navigate("login/create")}>Create Account</Button
          >
        </ButtonSet>
      </Column>
    </Row>
  </Grid>
</Content>
