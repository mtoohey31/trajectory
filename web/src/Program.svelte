<script lang="ts">
  import {
    Content,
    Button,
    Grid,
    Row,
    Column,
    Dropdown,
    TextInput,
  } from "carbon-components-svelte";
  import Centered from "./components/Centered.svelte";
  import type { Program } from "./classes";
  import * as Presets from "./gpa_table_presets";

  const presetList = Object.keys(Presets).map((k) => Presets[k]);
  const idPresets = presetList.reduce(
    (presetsSoFar, preset, i) =>
      presetsSoFar.concat([{ id: i, text: preset.friendlyName }]),
    []
  );

  let selectedPreset = 0;

  export let program: Program;
</script>

<Centered>
  <Content>
    <Grid>
      <Row>
        <Column>
          <h1>Program</h1>
          <h2>Institution Name</h2>
          <TextInput bind:value={program.institution} />
          <h2>GPA Table</h2>
          <Dropdown
            titleText="Presets"
            type="inline"
            items={idPresets}
            bind:selectedIndex={selectedPreset}
          />
          <Button
            size="field"
            on:click={() => {
              if (
                selectedPreset !== null &&
                selectedPreset < presetList.length
              ) {
                program.settings.GPATable = presetList[selectedPreset].table;
              }
            }}>Apply Preset</Button
          >
          <!-- TODO: allow for custom GPA tables -->
        </Column>
      </Row>
    </Grid>
  </Content>
</Centered>
