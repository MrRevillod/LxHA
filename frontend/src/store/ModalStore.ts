
import { create } from "zustand"

interface Modals {
    [key: string]: boolean;
}

type ModalVariant =   "fromUser"  | "fromAdmin" | "deleteInstance" | "deleteAccount" 
                    | "startInstance" | "stopInstance" | "rebuildInstance" | "rebootInstance" | null 

interface ModalStore {
    data?: any,
    modals: Modals,
    modalVariant?: ModalVariant,
    customAction?: (() => Promise<void>) | null | (() => any),
    closeAllModals: () => void,
    setModal: (
        name: string,
        data?: any,
        modalVariant?: ModalVariant,
        customAction?: (() => Promise<void>) | null | (() => any)
    ) => void,
}

export const useModalStore = create<ModalStore>((set, get) => ({

    modals: {
        "newAccount": false,
        "manageAccount": false,
        "editUser": false,
        "newInstance": false,
        "editInstance": false,
        "newMessage": false,
        "startInstance": false,
        "stopInstance": false,
        "rebootInstance": false,
        "rebuildInstance": false,
        "confirmAction": false,

    },

    data: null,

    setModal: (name: string, data: any = null, modalVariant: ModalVariant = null, customAction: (() => any) | null = null) => {

        set({ data, modalVariant, customAction })

        const newModals = { ...get().modals }
        newModals[name] = !newModals[name]

        set({ modals: newModals })
    },

    closeAllModals: () => {

        const newModals = { ...get().modals }

        for (const key in newModals) {
            newModals[key] = false
        }

        set({ modals: newModals })
    }

}))
