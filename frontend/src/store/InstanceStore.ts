import { api } from "../lib/axios"

import { create } from "zustand"
// import { instances } from "../lib/data"
import { InstanceData, Instance, PublicInstanceData } from "../lib/types"
import { useHttpStore } from "./HttpStore"

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
    startInstance: (id: string) => Promise<void>,
    stopInstance: (id: string) => Promise<void>,
    rebuildInstance: (id: string) => Promise<void>,
    restartInstance: (id: string) => Promise<void>,


    
}
const { setIsLoading, setResponse } = useHttpStore.getState()


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
        try {
            setIsLoading(true)
            const res = await api.get(`/dashboard/instances/${id}`)
            setResponse(res.status, res.data.message, res.data, true)

            return res.status

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, true)
            return error.response.status

        } finally {
            setIsLoading(false)
        }
    },

    getInstances: async () => {

        const res = await api.get("/dashboard/instances")
        const instances = res.data.instances;

        console.log(instances);

        const dataSplice = instances.slice(0, get().itemsPerPage);

        set({ data: instances, filteredData: instances, dataSplice });
    },

    createInstance: async (instance: InstanceData) => {
        try {
            // setIsLoading(true)
            const res = await api.post("/dashboard/instances", instance)
            setResponse(res.status, res.data.message, res.data, true)

            console.log(instance)

            return res.status

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, true)
            return error.response.status

        } finally {
            // setIsLoading(false)
        }
    },

    updateInstance: async (instance: InstanceData) => {
        return new Promise(() => instance)
    },

    deleteInstance: async (id: string) => {
        try {
            setIsLoading(true)
            const res = await api.delete(`/dashboard/instances/${id}`)
            setResponse(res.status, res.data.message, res.data, true)

            return res.status

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, true)
            return error.response.status

        } finally {
            setIsLoading(false)
        }
    },

    startInstance: async (id: string) => {
        try {
            setIsLoading(true)
            const res = await api.post(`/dashboard/instances/start/${id}`)
            setResponse(res.status, res.data.message, res.data, true)

            return res.status

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, true)
            return error.response.status

        } finally {
            setIsLoading(false)
        }
    },

    stopInstance: async (id: string) => {
        try {
            setIsLoading(true)
            const res = await api.post(`/dashboard/instances/delete/${id}`)
            setResponse(res.status, res.data.message, res.data, true)

            return res.status

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, true)
            return error.response.status

        } finally {
            setIsLoading(false)
        }
    },

    rebuildInstance: async (id: string) => {
        try {
            setIsLoading(true)
            const res = await api.post(`/dashboard/instances/rebuild/${id}`)
            setResponse(res.status, res.data.message, res.data, true)

            return res.status

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, true)
            return error.response.status

        } finally {
            setIsLoading(false)
        }
    },

    restartInstance: async (id: string) => {
        try {
            setIsLoading(true)
            const res = await api.post(`/dashboard/instances/restart/${id}`)
            setResponse(res.status, res.data.message, res.data, true)

            return res.status

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, true)
            return error.response.status

        } finally {
            setIsLoading(false)
        }
    },



}))