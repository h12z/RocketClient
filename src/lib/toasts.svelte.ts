export interface Toast {
    id: string;
    message: string;
    type: 'success' | 'error' | 'info';
    duration?: number;
}

class ToastManager {
    toasts = $state<Toast[]>([]);

    add(message: string, type: Toast['type'] = 'info', duration = 3000) {
        const id = crypto.randomUUID();
        this.toasts.push({ id, message, type, duration });

        if (duration > 0) {
            setTimeout(() => {
                this.remove(id);
            }, duration);
        }
    }

    remove(id: string) {
        this.toasts = this.toasts.filter((t) => t.id !== id);
    }
}

export const toastManager = new ToastManager();
