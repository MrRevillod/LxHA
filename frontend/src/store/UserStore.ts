
import { User } from "../lib/types"
import { users } from "../lib/data"
import { create } from "zustand"

export interface UserStore {
    getUsers: () => Promise<void>
}

export interface UserTableStore {
    data: User[]
    tableColumns: string[]
    nColumns: number
    currentPage: number
    itemsPerPage: number
    totalPages: number
    startIndex: number
    endIndex: number
    dataSplice: User[]
    searchTerm: string
    setSearchTerm: (term: string) => void
    setCurrentPage: (page: number) => void
    setItemsPerPage: (items: number) => void
    filterData: () => void
}

export const useUserStore = create<UserStore & UserTableStore>((set, get) => ({

    data: [],
    tableColumns: ["ID", "Name", "Username", "Role", "Instances", "Actions"],
    nColumns: 0,
    currentPage: 1,
    itemsPerPage: 10,
    totalPages: 0,
    startIndex: 0,
    endIndex: 0,
    dataSplice: [],
    searchTerm: "",

    getUsers: async () => {

        try {

            console.log("Fetching users...")

            const totalPages = Math.ceil(users.length / get().itemsPerPage)
            set({ data: users, totalPages, dataSplice: users.slice(0, get().itemsPerPage) })
            set({ nColumns: get().tableColumns.length })

        } catch (error: any) {
            console.error(error)
        }
    },

    setCurrentPage: (page: number) => {

        set({ currentPage: page })
        set({ totalPages: Math.ceil(get().data.length / get().itemsPerPage) })

        set({ startIndex: (page - 1) * get().itemsPerPage })
        set({ endIndex: get().startIndex + get().itemsPerPage })

        get().filterData()
    },

    setItemsPerPage: (items: number) => {

        set({ itemsPerPage: items })
        set({ totalPages: Math.ceil(get().data.length / items) })

        set({ startIndex: (get().currentPage - 1) * items })
        set({ endIndex: get().startIndex + items })

        get().filterData()
    },

    setSearchTerm: (term: string) => {
        set({ searchTerm: term, currentPage: 1 })
        get().filterData()
    },

    filterData: () => {

        const filteredData = get().data.filter(user =>
            user.name.toLowerCase().includes(get().searchTerm.toLowerCase()) ||
            user.username.toLowerCase().includes(get().searchTerm.toLowerCase()) ||
            user.role.toLowerCase().includes(get().searchTerm.toLowerCase())
        )

        const totalPages = Math.ceil(filteredData.length / get().itemsPerPage)
        const startIndex = (get().currentPage - 1) * get().itemsPerPage
        const endIndex = startIndex + get().itemsPerPage
        const dataSplice = filteredData.slice(startIndex, endIndex)

        set({ totalPages, dataSplice })
    }
}))
