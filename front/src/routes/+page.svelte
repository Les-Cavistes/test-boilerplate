<script lang="ts">
  // Imports
  import {onMount} from "svelte";
  import type {PageProps} from "../../.svelte-kit/types/src/routes/$types";
  import type {TTask} from "$lib/types";
  import {tasksStore} from '$lib/stores/tasks';
  import axios from "axios";
  import "$lib/styles/main.scss";

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
      const response = await axios.get(`/api/task/${taskId}/toggle`);

      if (!response.data || response.data.status !== 'success') {
        throw new Error('Failed to update task');
      }

      const result = response.data.task;

      // Update the task in the list
      tasksStore.toggleTask(result);
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

  const handleAddTask = async (event: Event) => {
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

  const handleDeleteTask = async (taskId: number) => {
    try {
      const response = await axios.delete(`/api/task/${taskId}`);

      if (response.data.status === 'success') {
        // Update the store with the new task
        tasksStore.deleteTask(taskId);
      } else {
        throw new Error(response.data.message || 'Failed to delete task');
      }
    } catch (error) {
      console.error('Error deleting task:', error);
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
          <button
              class="delete-button"
              onclick={() => handleDeleteTask(task.id)}
          >
            Delete
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</main>

<style lang="scss">
  // Variables
  $primary-color: #0066cc;
  $primary-hover: #0052a3;
  $danger-color: #ff4444;
  $danger-hover: #ff6666;
  $error-bg: #ffe6e6;
  $border-color: #eee;
  $text-primary: #333;
  $text-secondary: #444;
  $text-muted: #888;
  $transition-speed: 0.2s;

  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;

    header {
      margin-bottom: 2rem;
    }
  }

  h1 {
    font-size: 2rem;
    color: $text-primary;
  }

  // Task List Styles
  .task-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .task-item {
    padding: 1rem;
    border-bottom: 1px solid $border-color;
    transition: background-color $transition-speed;
    position: relative;

    &:hover {
      background-color: #f9f9f9;

      .delete-button {
        opacity: 1;
      }
    }

    // Delete Button
    .delete-button {
      height: 60%;
      position: absolute;
      right: 8px;
      top: 50%;
      transform: translateY(-50%);
      opacity: 0;
      transition: opacity $transition-speed;
      background: $danger-color;
      color: white;
      border: none;
      border-radius: 4px;
      padding: 4px 8px;
      cursor: pointer;
      font-size: 1rem;

      &:hover {
        background: $danger-hover;
      }
    }
  }

  .task-label {
    display: flex;
    align-items: center;
    gap: 1rem;
    cursor: pointer;
  }

  .task-description {
    font-size: 1.1rem;
    color: $text-secondary;

    input[type="checkbox"]:checked + & {
      text-decoration: line-through;
      color: $text-muted;
    }
  }

  input[type="checkbox"] {
    width: 1.2rem;
    height: 1.2rem;
  }

  .loading {
    font-size: 0.8rem;
    color: #666;
    font-style: italic;
  }

  // Form Styles
  .create-task-form {
    margin-bottom: 2rem;
    display: flex;
    gap: 1rem;

    input {
      flex: 1;
      padding: 0.5rem;
      font-size: 1rem;
      border: 1px solid #ddd;
      border-radius: 4px;
    }

    button {
      padding: 0.5rem 1rem;
      font-size: 1rem;
      background-color: $primary-color;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      transition: background-color $transition-speed;

      &:hover:not(:disabled) {
        background-color: $primary-hover;
      }

      &:disabled {
        background-color: #ccc;
        cursor: not-allowed;
      }
    }
  }


  // Error Message
  .error-message {
    color: #dc3545;
    margin-bottom: 1rem;
    padding: 0.5rem;
    border-radius: 4px;
    background-color: $error-bg;
  }
</style>
