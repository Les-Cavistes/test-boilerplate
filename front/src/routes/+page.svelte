<script lang="ts">
  // Imports
  import {onMount} from "svelte";
  import type {PageProps} from "../../.svelte-kit/types/src/routes/$types";
  import type {TTask} from "$lib/types";
  import {tasksStore} from '$lib/stores/tasks';
  import axios from "axios";

  // Variables
  const {data}: PageProps = $props();

  const tasks = $state<{
    tasks: TTask[],
    total: number,
    loading: Record<number, boolean>,
    creating: boolean,
  }>({
    tasks: [],
    total: 0,
    loading: {},
    creating: false,
  });

  let newTaskDescription = $state("");
  let errorMessage = $state("");

  // Functions
  const handleToggle = async (taskId: number, currentCompleted: boolean) => {
    // Set loading state for this task
    tasks.loading[taskId] = true;

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
      tasks.loading[taskId] = false;
    }
  }

  async function handleAddTask(event: Event) {
    event.preventDefault();

    if (!newTaskDescription.trim()) {
      errorMessage = 'Task description cannot be empty';
      return;
    }

    tasks.creating = true;
    errorMessage = '';

    try {
      const response = await axios.post('/api/task', {
        description: newTaskDescription.trim(),
      });

      if (response.data.status === 'success' && response.data.task) {
        // Update the store with the new task
        tasksStore.addTask(response.data.task);
        // Clear input
        newTaskDescription = '';
      } else {
        throw new Error(response.data.message || 'Failed to create task');
      }
    } catch (error) {
      console.error('Error creating task:', error);
      errorMessage = error instanceof Error ? error.message : 'Failed to create task';
    } finally {
      tasks.creating = false;
    }
  }

  // Lifecycle
  onMount(() => {
    console.log(data.tasks);
  })
</script>

<main class="container">
  <header>
    <h1>Tasks ({$tasksStore?.total ?? 0})</h1>
  </header>

  <form
      class="create-task-form"
      onsubmit={handleAddTask}
  >
    <input
        type="text"
        placeholder="Enter new task..."
        bind:value={newTaskDescription}
        disabled={tasks.creating}
    />
    <button
        type="submit"
        disabled={tasks.creating || !newTaskDescription.trim()}
    >
      {tasks.creating ? 'Creating...' : 'Add Task'}
    </button>
  </form>

  {#if errorMessage}
    <div class="error-message">
      {errorMessage}
    </div>
  {/if}

  {#if $tasksStore}
    <ul class="task-list">
      {#each $tasksStore.tasks as task (task.id)}
        <li class="task-item">
          <label class="task-label">
            <input
                type="checkbox"
                checked={task.completed}
                disabled={tasks.loading[task.id]}
                onchange={() => handleToggle(task.id, task.completed)}
            />
            <span class="task-description">
              {task.description}
            </span>
            {#if tasks.loading[task.id]}
              <span class="loading">updating...</span>
            {/if}
          </label>
        </li>
      {/each}
    </ul>
  {/if}
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

    .create-task-form {
        margin-bottom: 2rem;
        display: flex;
        gap: 1rem;
    }

    .create-task-form input {
        flex: 1;
        padding: 0.5rem;
        font-size: 1rem;
        border: 1px solid #ddd;
        border-radius: 4px;
    }

    .create-task-form button {
        padding: 0.5rem 1rem;
        font-size: 1rem;
        background-color: #0066cc;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .create-task-form button:hover:not(:disabled) {
        background-color: #0052a3;
    }

    .create-task-form button:disabled {
        background-color: #ccc;
        cursor: not-allowed;
    }

    .error-message {
        color: #dc3545;
        margin-bottom: 1rem;
        padding: 0.5rem;
        border-radius: 4px;
        background-color: #ffe6e6;
    }
</style>
