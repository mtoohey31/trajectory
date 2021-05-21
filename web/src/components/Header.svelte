<script lang="ts">
  import {
    SkipToContent,
    Header,
    HeaderUtilities,
    HeaderAction,
    HeaderPanelLinks,
    HeaderPanelLink,
    HeaderPanelDivider,
  } from "carbon-components-svelte";
  import UserAvatar20 from "carbon-icons-svelte/lib/UserAvatar20/UserAvatar20.svelte";
  import Renew20 from "carbon-icons-svelte/lib/Renew20/Renew20.svelte";
  import { navigate } from "svelte-routing";

  import type { UserData } from "../classes";

  export let userData: UserData;
  export let username: string;
  export let loginKey: CryptoKey;
  export let syncing: boolean;

  let isOpen;
  let popOverOpen;

  let isSideNavOpen = false;
</script>

<Header company="Trajectory" href="/" bind:isSideNavOpen>
  <div slot="skip-to-content">
    <SkipToContent />
  </div>
  <HeaderUtilities>
    {#if syncing}
      <div class="sync">
        <Renew20 on:mouseover={() => (popOverOpen = true)} />
      </div>
    {/if}
    <HeaderAction icon={UserAvatar20} bind:isOpen transition={false}>
      <HeaderPanelLinks>
        <HeaderPanelDivider>Account</HeaderPanelDivider>
        <HeaderPanelLink on:click={() => navigate("/account")}
          >Settings</HeaderPanelLink
        >
        <HeaderPanelLink href="/login">Log Out</HeaderPanelLink>
      </HeaderPanelLinks>
    </HeaderAction>
  </HeaderUtilities>
</Header>

<style>
  .sync {
    animation: rotate 1.5s linear infinite;
    margin-top: auto;
    margin-bottom: auto;
  }
  @keyframes rotate {
    to {
      transform: rotate(-359deg);
    }
  }
</style>
