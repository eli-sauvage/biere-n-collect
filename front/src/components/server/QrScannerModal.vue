<script setup lang="ts">
import QrScanner from 'qr-scanner'

import {
    get_order_by_receipt,
    type Order,
} from '../../scripts/api/admin/order-management'
import { watch } from 'vue'
import { onMounted } from 'vue'

let visible = defineModel<boolean>()
let emits = defineEmits<{ selectOrder: [order: Order | null] }>()

let qrScanner: QrScanner | null
watch(visible, async (visible) => {
    let video = document.getElementById(
        'serveurQrScannerVideoElem'
    ) as HTMLVideoElement
    if (visible == true) {
        video.style.display = 'unset'
        document
            .getElementById('serverQrScannerVideoContainer')
            ?.appendChild(video)
        qrScanner?.start()
    } else {
        video.style.display = 'none'
        document.body.appendChild(video)
        qrScanner?.stop()
    }
})

onMounted(() => {
    console.log('mounted')
    let video = document.createElement('video')
    video.id = 'serveurQrScannerVideoElem'
    video.style.display = 'none'
    document.body.appendChild(video)
    qrScanner = initScanner()
})

const initScanner = () => {
    qrScanner = new QrScanner(
        document.getElementById(
            'serveurQrScannerVideoElem'
        ) as HTMLVideoElement,
        async (result) => {
            let data = result.data
            if (
                data.match(
                    /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/
                )
            ) {
                console.log('decoded qr code:', data)
                if (qrScanner) qrScanner.stop()
                visible.value = false
                let order = await get_order_by_receipt(data)
                if (order == null) return
                emits('selectOrder', order)
            }
        },
        {
            highlightCodeOutline: true,
            highlightScanRegion: true,
        }
    )
    qrScanner.start()
    return qrScanner
}
</script>
<template>
    <Dialog
        modal
        header="Commande séléctionnée"
        v-model:visible="visible"
        :dismissableMask="true"
    >
        <div
            class="scan-container"
            :style="`display: ${visible ? 'unset' : 'none'}`"
            id="serverQrScannerVideoContainer"
        ></div>
    </Dialog>
</template>
<style scoped>
video {
    max-width: 90vw;
    max-height: 90vh;
}
</style>
