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

  export let institution: string;
  export let syncing: boolean;

  let isOpen: boolean;
</script>

<Header company="Trajectory" platformName={institution}>
  <div slot="skip-to-content">
    <SkipToContent />
  </div>
  <HeaderUtilities>
    {#if syncing}
      <div class="sync">
        <Renew20 style="fill: var(--cds-icon-03);" />
      </div>
    {/if}
    <HeaderAction icon={UserAvatar20} bind:isOpen transition={false}>
      <HeaderPanelLinks>
        <HeaderPanelDivider>Program</HeaderPanelDivider>
        <HeaderPanelLink on:click={() => navigate("/program")}
          >Settings</HeaderPanelLink
        >
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
    width: 20px;
    height: 20px;
  }
  @keyframes rotate {
    to {
      transform: rotate(-359deg);
    }
  }
</style>
