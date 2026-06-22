export interface Server {
    id: string;
    name: string;
    address: string;
}

class ServerStore {
    servers = $state<Server[]>([]);

    constructor() {
        if (typeof window !== 'undefined') {
            const saved = localStorage.getItem('rocket_servers');
            if (saved) {
                try {
                    this.servers = JSON.parse(saved);
                } catch (e) {
                    console.error('Failed to parse saved servers', e);
                    this.loadDefaults();
                }
            } else {
                this.loadDefaults();
            }
        }
    }

    private loadDefaults() {
        this.servers = [
            { id: crypto.randomUUID(), name: 'Hypixel', address: 'mc.hypixel.net' },
            { id: crypto.randomUUID(), name: 'Wynncraft', address: 'play.wynncraft.com' }
        ];
        this.save();
    }

    private save() {
        if (typeof window !== 'undefined') {
            localStorage.setItem('rocket_servers', JSON.stringify(this.servers));
        }
    }

    add(name: string, address: string) {
        this.servers.push({
            id: crypto.randomUUID(),
            name,
            address
        });
        this.save();
    }

    remove(id: string) {
        this.servers = this.servers.filter(s => s.id !== id);
        this.save();
    }

    update(id: string, name: string, address: string) {
        const index = this.servers.findIndex(s => s.id === id);
        if (index !== -1) {
            this.servers[index] = { ...this.servers[index], name, address };
            this.save();
        }
    }
}

export const serverStore = new ServerStore();
