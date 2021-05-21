<script lang="ts">
  import { themes } from "./Theme.svelte";
  import {
    SkipToContent,
    Header,
    SideNav,
    SideNavItems,
    SideNavLink,
    SideNavMenu,
    SideNavMenuItem,
    HeaderUtilities,
    HeaderGlobalAction,
  } from "carbon-components-svelte";
  import Light20 from "carbon-icons-svelte/lib/Light20/Light20.svelte";
  import Settings20 from "carbon-icons-svelte/lib/Settings20/Settings20.svelte";
  import Upload20 from "carbon-icons-svelte/lib/Upload20/Upload20.svelte";
  import UserAvatar20 from "carbon-icons-svelte/lib/UserAvatar20/UserAvatar20.svelte";
  import { navigate } from "svelte-routing";

  import { UserData, Program, UserSettings } from "../classes";

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
        navigate("/login");
      })
      .catch(() => {
        // TODO: Handle failed deletion errors here
      });
  }
  export let userData: UserData;
  export let username: string;
  export let loginKey: CryptoKey;
  export let updateData: () => void;

  export let theme;

  let isSideNavOpen = false;
</script>

<Header company="Trajectory" href="/" bind:isSideNavOpen>
  <div slot="skip-to-content">
    <SkipToContent />
  </div>
  <HeaderUtilities>
    <HeaderGlobalAction icon={Upload20} on:click={() => updateData()} />
    <HeaderGlobalAction icon={UserAvatar20} />
    <HeaderGlobalAction aria-label="Settings" icon={Settings20} />
  </HeaderUtilities>
</Header>

<SideNav bind:isOpen={isSideNavOpen}>
  <SideNavItems>
    <SideNavMenu text="Programs">
      {#each userData.programs as program}<SideNavMenuItem
          text={program.institution}
        />{/each}<SideNavMenuItem
        on:click={() => {
          userData.programs.push(new Program("", [], new UserSettings()));
        }}
        text="Add Program"
      />
    </SideNavMenu>
    <SideNavMenu text="Theme" icon={Light20}>
      {#each themes as currTheme}
        <SideNavMenuItem
          text={currTheme}
          on:click={() => (theme = currTheme)}
        />
      {/each}
    </SideNavMenu>
    <SideNavLink text="Delete Account" on:click={deleteAccount} />
  </SideNavItems>
</SideNav>
