import { writable } from 'svelte/store';
import type { TTask, TTasksResponse } from '$lib/types';

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
                total: state.total + 1
            };
        }),
        deleteTask: (id: number) => update(state => {
            if (!state) return state;

            return {
                ...state,
                tasks: state.tasks.filter(task => task.id !== id),
                total: state.total - 1
            };
        }),
        toggleTask: (id: number) => update(state => {
            if (!state) return state;

            return {
                ...state,
                tasks: state.tasks.map(task => {
                    if (task.id === id) {
                        return {
                            ...task,
                            completed: !task.completed
                        };
                    }

                    return task;
                })
            };
        })
    };
}

export const tasksStore = createTasksStore(); 