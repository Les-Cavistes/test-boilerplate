<script lang="ts">
  // Imports
  import {onMount} from "svelte";
  import type {PageProps} from "../../.svelte-kit/types/src/routes/$types";

  // Variables
  const {data}: PageProps = $props();
  let isLoading: Record<number, boolean> = {};

  // Functions
  async function handleToggle(taskId: number, currentCompleted: boolean) {
    // Set loading state for this task
    isLoading[taskId] = true;

    try {
      const response = await fetch(`/api/task/${taskId}`, {
        method: 'PUT',
      });

      if (!response.ok) {
        throw new Error('Failed to update task');
      }

      const result = await response.json();

      // Update the task in the list
      data.tasks.tasks = data.tasks.tasks.map(task =>
        task.id === taskId
          ? {...task, completed: result.completed}
          : task
      );
    } catch (error) {
      console.error('Error toggling task:', error);
      // Revert the checkbox state
      data.tasks.tasks = data.tasks.tasks.map(task =>
        task.id === taskId
          ? {...task, completed: currentCompleted}
          : task
      );
    } finally {
      // Clear loading state
      isLoading[taskId] = false;
    }
  }

  // Lifecycle
  onMount(() => {
    console.log(data.tasks);
  })
</script>

<main class="container">
  <header>
    <h1>Tasks ({data.tasks.total})</h1>
  </header>

  <ul class="task-list">
    {#each data.tasks.tasks as task (task.id)}
      <li class="task-item">
        <label class="task-label">
          <input
              type="checkbox"
              checked={task.completed}
              disabled={isLoading[task.id]}
              on:change={() => handleToggle(task.id, task.completed)}
          />
          <span class="task-description">
            {task.description}
          </span>
          {#if isLoading[task.id]}
            <span class="loading">updating...</span>
          {/if}
        </label>
      </li>
    {/each}
  </ul>
</main>

<style>
    .container {
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
    }

    header {
        margin-bottom: 2rem;
    }

    h1 {
        font-size: 2rem;
        color: #333;
    }

    .task-list {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    .task-item {
        padding: 1rem;
        border-bottom: 1px solid #eee;
        transition: background-color 0.2s;
    }

    .task-item:hover {
        background-color: #f9f9f9;
    }

    .task-label {
        display: flex;
        align-items: center;
        gap: 1rem;
        cursor: pointer;
    }

    .task-description {
        font-size: 1.1rem;
        color: #444;
    }

    input[type="checkbox"] {
        width: 1.2rem;
        height: 1.2rem;
    }

    input[type="checkbox"]:checked + .task-description {
        text-decoration: line-through;
        color: #888;
    }

    .loading {
        font-size: 0.8rem;
        color: #666;
        font-style: italic;
    }
</style>
