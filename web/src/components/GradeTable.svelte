<script lang="ts">
  import * as Classes from "../classes";
  import {
    Button,
    DataTable,
    NumberInput,
    Toolbar,
    ToolbarContent,
    ToolbarMenu,
    ToolbarMenuItem,
  } from "carbon-components-svelte";
  import DataTableContainer from "carbon-components-svelte/src/DataTable/TableContainer.svelte";
  import Add20 from "carbon-icons-svelte/lib/Add20/Add20.svelte";
  import Delete20 from "carbon-icons-svelte/lib/Delete20/Delete20.svelte";

  export let grade: Classes.Grade;
  let update = false;
</script>

{#key update}
  {#if grade.constructor.name == "WeightedAverageGrade"}
    <DataTable
      batchExpansion
      title="Weighted Average"
      description="Grades are calculated as a weighted average, based on the manually entered weighting value for each grade."
      headers={[
        { key: "name", value: "Name" },
        { key: "grade", value: "Grade" },
        { key: "weight", value: "Weight" },
      ]}
      rows={grade.grades.map((grade, i) => {
        grade.id = i;
        let predicted = grade.predicted();
        grade.grade = predicted ? (predicted * 100).toFixed(2) + "%" : "--";
        return grade;
      })}
    >
      <span slot="cell" let:cell let:row>
        {#if cell.key === "weight"}
          <NumberInput
            min={0}
            step={0.01}
            bind:value={grade.weights[row.id]}
            style="width: 5rem;"
          />
        {:else}
          {cell.value}
        {/if}
      </span>
      <div slot="expanded-row" let:row>
        <svelte:self bind:grade={grade.grades[row.id]} />
      </div>
      <Toolbar>
        <ToolbarContent>
          <ToolbarMenu icon={Add20}>
            <ToolbarMenuItem
              on:click={() => {
                grade.grades.push(new Classes.PercentGrade("New Grade", ""));
                update = !update;
              }}>Percentage Grade</ToolbarMenuItem
            >
            <ToolbarMenuItem
              on:click={() => {
                grade.grades.push(new Classes.FractionGrade("New Grade", 0, 0));
                update = !update;
              }}>Fraction Grade</ToolbarMenuItem
            >
            <ToolbarMenuItem
              on:click={() => {
                grade.grades.push(new Classes.AverageGrade("New Grade", []));
                update = !update;
              }}>Average Grade</ToolbarMenuItem
            >
          </ToolbarMenu>
        </ToolbarContent>
      </Toolbar>
    </DataTable>
  {:else if grade.constructor.name == "AverageGrade"}
    <DataTable
      title="Average Grade"
      description="Grades are calculated as a simple average, with each grade having equal weighting."
      batchExpansion
      headers={[
        { key: "name", value: "Name" },
        { key: "grade", value: "Grade" },
      ]}
      rows={grade.grades.map((grade, i) => {
        grade.id = i;
        let predicted = grade.predicted();
        grade.grade = predicted ? (predicted * 100).toFixed(2) + "%" : "--";
        return grade;
      })}
    >
      <span slot="cell" let:cell let:row>
        {cell.value}
      </span>
      <div slot="expanded-row" let:row>
        <svelte:self bind:grade={grade.grades[row.id]} />
      </div>
      <Toolbar>
        <ToolbarContent>
          <ToolbarMenu icon={Add20}>
            <ToolbarMenuItem
              on:click={() => {
                grade.grades.push(new Classes.PercentGrade("New Grade", ""));
                update = !update;
              }}>Percentage Grade</ToolbarMenuItem
            >
            <ToolbarMenuItem
              on:click={() => {
                grade.grades.push(new Classes.FractionGrade("New Grade", 0, 0));
                update = !update;
              }}>Fraction Grade</ToolbarMenuItem
            >
            <ToolbarMenuItem
              on:click={() => {
                grade.grades.push(new Classes.AverageGrade("New Grade", []));
                update = !update;
              }}>Average Grade</ToolbarMenuItem
            >
          </ToolbarMenu>
          <Button icon={Delete20} kind="danger" iconDescription="Delete" />
        </ToolbarContent>
      </Toolbar>
    </DataTable>
  {:else if grade.constructor.name === "FractionGrade"}
    <DataTableContainer
      title="Fraction Grade"
      description="The grade is calculated as the quotient of the numerator and denominator."
    >
      <NumberInput
        style="display: inline;"
        label="Numerator"
        bind:value={grade.numerator}
      /><NumberInput
        style="display: inline;"
        label="Denominator"
        bind:value={grade.denominator}
      />
    </DataTableContainer>
  {:else if grade.constructor.name === "PercentGrade"}
    <DataTableContainer
      title="Percentage Grade"
      description="The grade is the percentage entered."
    >
      <NumberInput label="Percentage" step={0.01} bind:value={grade.percent} />
    </DataTableContainer>
  {/if}
{/key}
