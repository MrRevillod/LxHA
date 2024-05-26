
import { create } from "zustand"
import { toUpperAndLower } from "../lib/string"
// import { persist, createJSONStorage } from 'zustand/middleware'

interface AppStore {
    pageTitle: string,
    setPageTitle: (pageTitle: string) => void
    reset(): void,
}

const getInitialPageTitle = () => {

    const path = window.location.pathname.split("/")[1]

    const pages = ["users", "analytics", "instances", "dashboard"]

    if (pages.includes(path)) {
        return toUpperAndLower(path)
    }

    return "Dashboard"
}

export const useAppStore = create<AppStore>((set) => ({
    pageTitle: getInitialPageTitle(),
    setPageTitle: (pageTitle: string) => set({ pageTitle }),
    reset: () => set({ pageTitle: getInitialPageTitle() }),
}))


// export const useAppStore = create(

//     persist<AppStore>((set) => ({

//         pageTitle: getInitialPageTitle(),
//         setPageTitle: (pageTitle: string) => set({ pageTitle }),
//     }),

//         {
//             name: 'app-store',
//             storage: createJSONStorage(() => localStorage)
//         },
//     ),
// )
