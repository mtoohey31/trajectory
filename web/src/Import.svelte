<script lang="ts">
  import {
    Button,
    Content,
    Grid,
    Row,
    Column,
    FileUploaderButton,
  } from "carbon-components-svelte";
  import Centered from "./components/Centered.svelte";
  import { UserData } from "./classes";
  import { navigate } from "svelte-routing";

  let files;
  export let userData: UserData;

  async function uploadData(event: Event) {
    userData = UserData.from(JSON.parse(await event.target.files[0].text()));
    history.back();
  }
</script>

<Centered>
  <Content>
    <Grid>
      <Row>
        <Column>
          <h1>Import</h1>
          <FileUploaderButton
            labelText="Upload Data"
            accept={["application/json"]}
            on:change={uploadData}
          />
        </Column>
      </Row>
    </Grid>
  </Content>
</Centered>
