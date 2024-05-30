
import { User } from "../lib/types"
import { users } from "../lib/data"
import { create } from "zustand"

<<<<<<< HEAD
export interface UserStore {
    data: User[],
    dataSplice: User[],
    filteredData: User[],
    itemsPerPage: number,
    nColumns: number,
    columns: string[],
    setSplicedData: (page: number) => void,
    filterBySearch: (search: string) => void
}

export interface UserActions {
    getUsers: () => void
}
=======
interface UserStore {

    user: User | null,
    setUser: (user: User | null) => void,
    createUser: (userData:RegisterData) => Promise<void>,
    updateUser: () => Promise<void>,
    deleteUser: (id: string) => Promise<void>,
}

export const  useUserStore = create<UserStore>((set) => ({
>>>>>>> dev-auth

export const useUserStore = create<UserStore & UserActions>((set, get) => ({

    data: [],
    dataSplice: [],
    filteredData: [],
    itemsPerPage: 10,

<<<<<<< HEAD
    nColumns: 7,
    columns: ["ID", "NAME", "USERNAME", "EMAIL", "ROLE", "INSTANCES", "ACTIONS"],

    setSplicedData: (pageNumber: number) => {

        const startIndex = (pageNumber - 1) * get().itemsPerPage
        const endIndex = Math.min(startIndex + get().itemsPerPage, get().filteredData.length)

        set({ dataSplice: get().filteredData.slice(startIndex, endIndex) })
=======
    createUser: async (userData : RegisterData) => {

        // const userData: RegisterData = {
        //     email: "mail_test@mail.com",
        //     username: "test_user",
        //     role: ROLE.ADMINISTRATOR,
        //     password: "aaa",
        //     confirmPassword: "aaa"
        // }

        console.log("userData: ", userData)
        try {

            await api.post("/dashboard/user/register-account", userData)

        } catch (error: any) {
            console.error(error)
        }
>>>>>>> dev-auth
    },

    getUsers: async () => {

        const fetchedMessages = users
        const dataSplice = fetchedMessages.slice(0, get().itemsPerPage);

        set({ data: fetchedMessages, filteredData: fetchedMessages, dataSplice });
    },

    filterBySearch: (search: string) => {

        const filteredData = get().data.filter((user: User) =>
            user.name.toLowerCase().includes(search.toLowerCase()) ||
            user.username.toLowerCase().includes(search.toLowerCase()) ||
            user.role.toLowerCase().includes(search.toLowerCase()) ||
            user.email.toLowerCase().includes(search.toLowerCase())
        )

        set({ filteredData, dataSplice: filteredData.slice(0, get().itemsPerPage) })
    }

}))
