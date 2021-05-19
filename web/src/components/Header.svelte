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
  import UserAvatar20 from "carbon-icons-svelte/lib/UserAvatar20/UserAvatar20.svelte";

  import { UserData, Program } from "../classes";

  async function deleteAccount() {
    const exportedKey = await window.crypto.subtle.exportKey(
      "raw",
      hashedPasswd
    );
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
    fetch("/api/users", options);
  }
  export let userData: UserData;
  export let username: string;
  export let hashedPasswd: CryptoKey;

  export let theme;

  let isSideNavOpen = false;
</script>

<Header company="Trajectory" href="/" bind:isSideNavOpen>
  <div slot="skip-to-content">
    <SkipToContent />
  </div>
  <HeaderUtilities>
    <HeaderGlobalAction icon={UserAvatar20} />
    <HeaderGlobalAction aria-label="Settings" icon={Settings20} />
  </HeaderUtilities>
</Header>

<SideNav bind:isOpen={isSideNavOpen}>
  <SideNavItems>
    <SideNavMenu text="Programs">
      {#each userData.programs as program, i}<SideNavMenuItem
          on:click={() => console.log(i)}
          text={program.institution}
        />{/each}<SideNavMenuItem
        on:click={() => {
          userData.programs.push(new Program("", []));
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
