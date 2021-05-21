<script lang="ts">
  import * as Classes from "../classes";
  import {
    Button,
    Checkbox,
    DataTable,
    NumberInput,
    Search,
    TextInput,
    Toolbar,
    ToolbarContent,
  } from "carbon-components-svelte";
  import Add20 from "carbon-icons-svelte/lib/Add20/Add20.svelte";
  import GradeTable from "./GradeTable.svelte";

  export let program: Classes.Program;
  let searchString: string;
  $: gradeStrings = program.courses.map((course) => {
    return (course.predicted() * 100).toFixed(2).toString();
  });
</script>

<main>
  <DataTable
    batchExpansion
    headers={[
      { key: "name", value: "Name" },
      { key: "code", value: "Code" },
      { key: "grade", value: "% Grade" },
      { key: "finished", value: "Finished" },
    ]}
    rows={searchString
      ? program.courses
          .filter((course) => {
            return (
              course.name.indexOf(searchString) !== -1 ||
              course.code.indexOf(searchString) !== -1
            );
          })
          .map((course, i) => {
            course.id = i;
            let predicted = course.predicted();
            course.grade = predicted ? (predicted * 100).toFixed(2) : "--";
            return course;
          })
      : program.courses.map((course, i) => {
          course.id = i;
          let predicted = course.predicted();
          course.grade = predicted ? (predicted * 100).toFixed(2) : "--";
          return course;
        })}
  >
    <span slot="cell" let:cell let:row>
      {#if cell.key === "name"}
        <TextInput bind:value={program.courses[row.id][cell.key]} />
      {:else if cell.key === "code"}
        <TextInput
          bind:value={program.courses[row.id][cell.key]}
          style="width: 6rem;"
        />
      {:else if cell.key === "finished"}
        <Checkbox
          bind:checked={program.courses[row.id][cell.key]}
          on:click={() => {
            if (program.courses[row.id].finalGrade.percent === null) {
              program.courses[row.id].finalGrade.percent =
                program.courses[row.id].predicted() * 100;
            }
          }}
        />
      {:else if cell.key === "grade" && program.courses[row.id].finished}
        <NumberInput
          min={0}
          step={0.01}
          bind:value={program.courses[row.id].finalGrade.percent}
          style="width: 5rem;"
        />
      {:else if cell.key === "grade"}
        <NumberInput
          disabled
          value={cell.value}
          style="--cds-disabled-02: var(--cds-text-01); width: 5rem;"
        />
      {:else}
        {cell.value}
      {/if}
    </span>
    <div slot="expanded-row" let:row>
      <GradeTable bind:grade={program.courses[row.id].rootGrade} />
    </div>
    <Toolbar>
      <ToolbarContent>
        <Search bind:value={searchString} />
        <Button
          icon={Add20}
          on:click={() =>
            (program = new Classes.Program(
              program.institution,
              program.courses.concat([
                new Classes.Course(
                  "",
                  "",
                  new Classes.WeightedAverageGrade("", [], []),
                  false,
                  new Classes.PercentGrade("", null)
                ),
              ]),
              program.settings
            ))}>Add Course</Button
        >
      </ToolbarContent>
    </Toolbar>
  </DataTable>
</main>
