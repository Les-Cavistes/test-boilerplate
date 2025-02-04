import { writable } from 'svelte/store';
import type { TTask, TTasksResponse } from '$lib/types';

function createTasksStore() {
    const { subscribe, set, update } = writable<TTasksResponse>();

    return {
        subscribe,
        set: (value: TTasksResponse) => {
            // Sort tasks by descending ID when setting initial value
            const sortedTasks = {
                ...value,
                tasks: [...value.tasks].sort((a, b) => b.id - a.id)
            };
            set(sortedTasks);
        },
        addTask: (task: TTask) => update(state => {
            if (!state) return state;
            
            return {
                ...state,
                tasks: [task, ...state.tasks], // Add new task at the beginning
                total: state.total + 1
            };
        })
    };
}

export const tasksStore = createTasksStore(); 