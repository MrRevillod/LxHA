
import { Message } from "./types"
import { Instance, ROLE, User } from "./types"

export const users: User[] = [
    { id: "1u", name: "Alice Johnson", email: "alice.johnson@example.com", username: "AJohnson", role: ROLE.USER, n_instances: 2 },
    { id: "2u", name: "Luciano Revillod", email: "lrevillod@example.com", username: "lrevillod", role: ROLE.ADMINISTRATOR, n_instances: 0 },
    { id: "3u", name: "Carol White", email: "carol.white@example.com", username: "CWhite", role: ROLE.USER, n_instances: 1 },
    { id: "4u", name: "Albert Johnson", email: "albert.johnson@example.com", username: "AlbertJ", role: ROLE.USER, n_instances: 1 },
    { id: "5u", name: "Bart Sons", email: "b.sons@example.com", username: "BSons", role: ROLE.USER, n_instances: 1 },
    { id: "6u", name: "Carl Brown", email: "carl.b@example.com", username: "CBrown", role: ROLE.USER, n_instances: 2 },
    { id: "7u", name: "David Brown", email: "david.brown@example.com", username: "DBrown", role: ROLE.USER, n_instances: 1 },
    { id: "8u", name: "Tina Allen", email: "tina.allen@example.com", username: "TAllen", role: ROLE.USER, n_instances: 1 },
    { id: "9u", name: "Martin King", email: "m.king@example.com", username: "MKing", role: ROLE.USER, n_instances: 1 },
    { id: "10u", name: "Second Martin Queen", email: "m.queen@example.com", username: "MQueen", role: ROLE.USER, n_instances: 0 },
    { id: "11u", name: "Second Alice Johnson", email: "alice.johnson@example.com", username: "AJohnson", role: ROLE.USER, n_instances: 2 },
    { id: "21u", name: "Second Luciano Revillod", email: "lrevillod@example.com", username: "lrevillod", role: ROLE.ADMINISTRATOR, n_instances: 0 },
    { id: "31u", name: "Second Carol White", email: "carol.white@example.com", username: "CWhite", role: ROLE.USER, n_instances: 1 },
    { id: "41u", name: "Second Albert Johnson", email: "albert.johnson@example.com", username: "AlbertJ", role: ROLE.USER, n_instances: 1 },
    { id: "51u", name: "Second Bart Sons", email: "b.sons@example.com", username: "BSons", role: ROLE.USER, n_instances: 1 },
    { id: "61u", name: "Second Carl Brown", email: "carl.b@example.com", username: "CBrown", role: ROLE.USER, n_instances: 2 },
    { id: "71u", name: "Second David Brown", email: "david.brown@example.com", username: "DBrown", role: ROLE.USER, n_instances: 1 },
    { id: "81u", name: "Second Tina Allen", email: "tina.allen@example.com", username: "TAllen", role: ROLE.USER, n_instances: 1 },
    { id: "91u", name: "Second Martin King", email: "m.king@example.com", username: "MKing", role: ROLE.USER, n_instances: 1 },
    { id: "110u", name: "Martin Queen", email: "m.queen@example.com", username: "MQueen", role: ROLE.USER, n_instances: 0 },
]

export const instances: Instance[] = [
    { id: "1i", type: "Container", status: "Running", name: "My Instance 1", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "1u" },
    { id: "2i", type: "VM", status: "Running", name: "My Instance 2", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "1u" },
    { id: "3i", type: "Container", status: "Stopped", name: "My Instance 3", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "3u" },
    { id: "4i", type: "Container", status: "Running", name: "My Instance 4", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "4u" },
    { id: "5i", type: "Container", status: "Rebooting", name: "My Instance 5", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "3", user_id: "5u" },
    { id: "6i", type: "VM", status: "Running", name: "My Instance 6", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "6u" },
    { id: "7i", type: "Container", status: "Stopped", name: "My Instance 7", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "6u" },
    { id: "8i", type: "VM", status: "Running", name: "My Instance 8", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "3", user_id: "7u" },
    { id: "9i", type: "Container", status: "Running", name: "My Instance 9", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "8u" },
    { id: "10i", type: "Container", status: "Running", name: "My Instance 10", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "9u" },
    { id: "11i", type: "Container", status: "Running", name: "My Instance 1", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "1u" },
    { id: "21i", type: "VM", status: "Running", name: "My Instance 2", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "1u" },
    { id: "31i", type: "Container", status: "Stopped", name: "My Instance 3", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "3u" },
    { id: "41i", type: "Container", status: "Running", name: "My Instance 4", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "4u" },
    { id: "51i", type: "Container", status: "Rebooting", name: "My Instance 5", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "3", user_id: "5u" },
    { id: "61i", type: "VM", status: "Running", name: "My Instance 6", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "6u" },
    { id: "71i", type: "Container", status: "Stopped", name: "My Instance 7", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "6u" },
    { id: "81i", type: "VM", status: "Running", name: "My Instance 8", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "3", user_id: "7u" },
    { id: "91i", type: "Container", status: "Running", name: "My Instance 9", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "8u" },
    { id: "110i", type: "Container", status: "Running", name: "My Instance 10", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "9u" }
]

export const messages: Message[] = [
    { id: "1", subject: "General support", from: "John Doe", message: "Hello, how are you Hello, how are youHello, how are youHello, how are youHello, how are youHello, how are youHello, how are youHello, how are youHello, how are youHello, how are youHello, how are youHello, how are youHello, how are youHello, how are youHello, how are you?", date: "2021-09-01" },
    { id: "2", subject: "General support", from: "Jane Doe", message: "I'm fine, thank you", date: "2021-09-02" },
    { id: "3", subject: "General support", from: "John Doe", message: "I'm glad to hear that", date: "2021-09-03" },
    { id: "4", subject: "General support", from: "Jane Doe", message: "How about you?", date: "2021-09-04" },
    { id: "5", subject: "General support", from: "John Doe", message: "I'm fine too", date: "2021-09-05" },
    { id: "6", subject: "General support", from: "Jane Doe", message: "That's great", date: "2021-09-06" },
    { id: "7", subject: "General support", from: "John Doe", message: "Yes, it is", date: "2021-09-07" },
    { id: "8", subject: "General support", from: "Jane Doe", message: "I have to go now", date: "2021-09-08" },
    { id: "9", subject: "General support", from: "John Doe", message: "Ok, see you later", date: "2021-09-09" },
    { id: "10", subject: "General support", from: "Jane Doe", message: "Bye", date: "2021-09-10" },
    { id: "11", subject: "General support", from: "Second John Doe", message: "Hello, how are you?", date: "2021-09-01" },
    { id: "21", subject: "General support", from: "Second Jane Doe", message: "I'm fine, thank you", date: "2021-09-02" },
    { id: "31", subject: "General support", from: "Second John Doe", message: "I'm glad to hear that", date: "2021-09-03" },
    { id: "41", subject: "General support", from: "Second Jane Doe", message: "How about you?", date: "2021-09-04" },
    { id: "51", subject: "General support", from: "Second John Doe", message: "I'm fine too", date: "2021-09-05" },
    { id: "61", subject: "General support", from: "Second Jane Doe", message: "That's great", date: "2021-09-06" },
    { id: "71", subject: "General support", from: "Second John Doe", message: "Yes, it is", date: "2021-09-07" },
    { id: "81", subject: "General support", from: "Second Jane Doe", message: "I have to go now", date: "2021-09-08" },
    { id: "91", subject: "General support", from: "Second John Doe", message: "Ok, see you later", date: "2021-09-09" },
    { id: "10", subject: "General support", from: "Second Jane Doe", message: "Bye", date: "2021-09-10" }
]