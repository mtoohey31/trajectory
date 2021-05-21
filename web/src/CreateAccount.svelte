<script lang="ts">
  import {
    Button,
    Content,
    Grid,
    Row,
    Column,
    TextInput,
    PasswordInput,
  } from "carbon-components-svelte";
  import Add20 from "carbon-icons-svelte/lib/Add20/Add20.svelte";
  import { navigate } from "svelte-routing";

  async function createUser() {
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
    const exportedKey = await window.crypto.subtle.exportKey("raw", derivedKey);
    const exportedKeyBuffer = new Uint8Array(exportedKey);
    let dec = new TextDecoder();

    const options = {
      method: "POST",
      body: JSON.stringify({
        username: enteredUsername,
        hashed_passwd: dec.decode(exportedKeyBuffer),
      }),
      headers: {
        "Content-Type": "application/json",
      },
    };
    fetch("/api/users", options).then(async (res) => {
      if (res.status === 200) {
        navigate("/login");
      } else if (res.status === 409) {
        usernameInvalid = true;
      } else {
        invalidPasswordText = "An unknown error occurred";
      }
    });
  }

  let enteredUsername = "";
  let password = "";
  let passwordConfirmation = "";
  let enteringPassword = false;
  $: invalidUsernameText =
    enteredUsername.indexOf(":") === -1
      ? "Username is already taken"
      : "Username cannot contain :";
  $: usernameInvalid = enteredUsername.indexOf(":") !== -1;
  $: passwordMismatch = password !== passwordConfirmation;
  $: invalidPasswordText = passwordMismatch
    ? "Passwords do not match"
    : "Unknown error";
  $: createDisabled = usernameInvalid && invalidPasswordText;
</script>

<svelte:window
  on:keydown={(event) => {
    if (enteringPassword && event.key == "Enter") {
      createUser();
    }
  }}
/>

<Content>
  <Grid>
    <Row>
      <Column>
        <h1>Create Account</h1>
        <TextInput
          autofocus
          labelText="Username"
          bind:value={enteredUsername}
          bind:invalid={usernameInvalid}
          bind:invalidText={invalidUsernameText}
        />
        <PasswordInput
          labelText="Password"
          bind:value={password}
          bind:invalid={passwordMismatch}
          on:focus={() => (enteringPassword = true)}
          on:blur={() => (enteringPassword = false)}
        />
        <PasswordInput
          labelText="Confirm Password"
          bind:value={passwordConfirmation}
          bind:invalid={passwordMismatch}
          bind:invalidText={invalidPasswordText}
          on:focus={() => (enteringPassword = true)}
          on:blur={() => (enteringPassword = false)}
        />
        <Button
          style="margin-top: 1rem;"
          bind:disabled={createDisabled}
          icon={Add20}
          on:click={() => createUser()}>Create Account</Button
        >
      </Column>
    </Row>
  </Grid>
</Content>
