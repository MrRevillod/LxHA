
import { create } from "zustand"
import { Instance, PublicInstanceData } from "../lib/types"

interface InstanceStore {
    instances: Instance[],
    useGetInstances: () => Promise<void>,
    useGetInstance: (id: string) => Promise<void>,
    useCreateInstance: (instance: PublicInstanceData) => Promise<void>,
    useUpdateInstance: (instance: PublicInstanceData) => Promise<void>,
    useDeleteInstance: (id: string) => Promise<void>,
}

export const useInstanceStore = create<InstanceStore>(() => ({

    instances: [],

    useGetInstance: async (id: string) => {
        return new Promise(() => id)
    },

    useGetInstances: async () => {
        return new Promise(() => { })
    },

    useCreateInstance: async (instance: PublicInstanceData) => {
        return new Promise(() => instance)
    },

    useUpdateInstance: async (instance: PublicInstanceData) => {
        return new Promise(() => instance)
    },

    useDeleteInstance: async (id: string) => {
        return new Promise(() => id)
    }

}))