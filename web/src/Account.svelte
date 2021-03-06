<script lang="ts">
  import {
    Button,
    ButtonSet,
    Content,
    Dropdown,
    Grid,
    Row,
    Column,
    InlineNotification,
    OverflowMenu,
    OverflowMenuItem,
  } from "carbon-components-svelte";
  import Centered from "./components/Centered.svelte";
  import type { UserData } from "./classes";
  import { themes, themeNames } from "./components/Theme.svelte";
  import { navigate } from "svelte-routing";
  import Delete20 from "carbon-icons-svelte/lib/Delete20/Delete20.svelte";

  const idThemes = themes.reduce(
    (themesSoFar, _, i) => themesSoFar.concat([{ id: i, text: themeNames[i] }]),
    []
  );
  $: selectedIndex =
    themes.indexOf(userData.settings.theme) !== -1
      ? themes.indexOf(userData.settings.theme)
      : 0;

  export let userData: UserData;
  export let username: string;
  export let loginKey: CryptoKey;

  function exportData() {
    const a = document.createElement("a");
    a.href = URL.createObjectURL(
      new Blob([JSON.stringify(userData)], { type: "application/json" })
    );
    a.download = "Trajectory.json";
    a.click();
  }

  async function deleteAccount() {
    const exportedKey = await window.crypto.subtle.exportKey("raw", loginKey);
    let dec = new TextDecoder();
    let auth =
      "Basic " +
      window.btoa(
        unescape(encodeURIComponent(`${username}:${dec.decode(exportedKey)}`))
      );
    const options = {
      method: "DELETE",
      headers: {
        Authorization: auth,
        "Content-Type": "application/json",
      },
    };
    fetch("/api/users", options)
      .then(() => {
        navigate("/login?message=Account successfully deleted.");
      })
      .catch(() => {
        // TODO: Handle failed deletion errors here
      });
  }
</script>

<Centered>
  <Content>
    <Grid>
      <Row>
        <Column>
          <h1>Account</h1>
          <h2>Settings</h2>
          <Dropdown
            titleText="Theme"
            {selectedIndex}
            items={idThemes}
            on:select={(e) => {
              userData.settings.theme = themes[e.detail.selectedIndex];
            }}
          />
          <ButtonSet style="margin-top: 1rem; margin-bottom: 1rem;">
            <Button on:click={() => navigate("/account/import")}
              >Import Data</Button
            >
            <Button on:click={exportData}>Export Data</Button>
          </ButtonSet>
          <h2>Delete</h2>
          <OverflowMenu
            style="width: auto; margin-top: 1rem; margin-bottom: 1rem;"
          >
            <Button slot="menu" kind="danger" icon={Delete20}
              >Delete Account</Button
            >
            <OverflowMenuItem danger text="Confirm" on:click={deleteAccount} />
          </OverflowMenu>
          <InlineNotification
            hideCloseButton
            title="Warning:"
            subtitle="This action is irreversible, please consider exporting data first."
          />
        </Column>
      </Row>
    </Grid>
  </Content>
</Centered>
