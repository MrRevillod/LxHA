
import { create } from "zustand"
// import { instances } from "../lib/data"
import { api } from "../lib/axios"
import { InstanceData, Instance, PublicInstanceData } from "../lib/types"

export interface InstanceStore {
    data: Instance[],
    dataSplice: Instance[],
    filteredData: Instance[],
    itemsPerPage: number,
    columns: string[],
    nColumns: number,
    setSplicedData: (page: number) => void,
    filterBySearch: (search: string) => void
}

export interface InstanceActions {
    getInstances: () => Promise<void>,
    getInstance: (id: string) => Promise<void>,
    createInstance: (instance: InstanceData) => Promise<void>,
    updateInstance: (instance: InstanceData) => Promise<void>,
    deleteInstance: (id: string) => Promise<void>,
}

export const useInstanceStore = create<InstanceStore & InstanceActions>((set, get) => ({

    data: [],
    dataSplice: [],
    filteredData: [],
    itemsPerPage: 10,

    nColumns: 7,
    columns: ["STATUS", "NAME", "CPU'S", "MEMORY", "TYPE", "IPV4", "ACTIONS"],

    setSplicedData: (pageNumber: number) => {

        const startIndex = (pageNumber - 1) * get().itemsPerPage
        const endIndex = Math.min(startIndex + get().itemsPerPage, get().filteredData.length)

        set({ dataSplice: get().filteredData.slice(startIndex, endIndex) })
    },

    filterBySearch: (search: string) => {

        const filteredData = get().data.filter(instance =>
            instance.name.toLowerCase().includes(search.toLowerCase()) ||
            instance.ip_addresses.includes(search.toLowerCase()) ||
            instance.user_id.toLowerCase().includes(search.toLowerCase())
        )

        set({ filteredData, dataSplice: filteredData.slice(0, get().itemsPerPage) })
    },

    getInstance: async (id: string) => {
        return new Promise(() => id)
    },

    getInstances: async () => {

        const res = await api.get("/dashboard/instances")
        const instances = res.data.instances;

        console.log(instances);

        const dataSplice = instances.slice(0, get().itemsPerPage);

        set({ data: instances, filteredData: instances, dataSplice });
    },

    createInstance: async (instance: InstanceData) => {
        console.log(instance)
        return new Promise(() => instance)
    },

    updateInstance: async (instance: InstanceData) => {
        return new Promise(() => instance)
    },

    deleteInstance: async (id: string) => {
        return new Promise(() => id)
    }

}))