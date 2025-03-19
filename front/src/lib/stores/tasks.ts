import { writable } from 'svelte/store';
import type {TTask, TTasksResponse} from "$lib/types/task";

const createTasksStore = () => {
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
            };
        }),
        deleteTask: (id: number) => update(state => {
            if (!state || !state.tasks.length) return state;

            return {
                ...state,
                tasks: state.tasks.filter(task => task.id !== id),
            };
        }),
        updateTask: (task: TTask) => update(state => {
            if (!state) return state;

            return {
                ...state,
                tasks: state.tasks.map(t => t.id === task.id ? task : t)
            };
        }),
    };
}

export const tasksStore = createTasksStore(); 