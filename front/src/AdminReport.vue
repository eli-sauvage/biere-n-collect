<script setup lang="ts">
import html2pdf from "html2pdf.js"
import type { Ref } from 'vue';
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import { get_report, type ReportItem } from './scripts/api/admin/reports';
import Button from "primevue/button";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Row from 'primevue/row';
import ColumnGroup from 'primevue/columngroup';
import { get_stock } from './scripts/api/products';
import { f_price } from './scripts/utils';
const route = useRoute()

let report: Ref<ReportItem[] | null> = ref(null);
let dates: Ref<[Date, Date] | null> = ref(null);

(async ()=>{
  if(typeof route.query.begin != "string" || typeof route.query.end != "string")
    return
  let begin_raw = parseInt(route.query.begin);
  let end_raw = parseInt(route.query.end);
  if(isNaN(begin_raw) || isNaN(end_raw)) return

  let begin = new Date(begin_raw);
  let end = new Date(end_raw);
  dates.value = [begin, end]

  report.value = await get_report(begin, end);
})();
function exportToPDF(){
  if(dates.value == null)
    return
  const container = document.querySelector("#report-container")
  const fmt = new Intl.DateTimeFormat('fr-FR', {
    day: "2-digit",
    year: "numeric",
    month: "short",
    hour: "2-digit",
    minute: "2-digit"
  });
  const begin_fmt = fmt.format(dates.value[0]).replace(":", "h").replace(/ /g, "").replace(",", "_");
  const end_fmt = fmt.format(dates.value[1]).replace(":", "h").replace(/ /g, "").replace(",", "_");
  const opt = {
    margin: 10,
    filename: `rapport-${begin_fmt}-${end_fmt}.pdf`,
    jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' }
  };
  html2pdf().from(container).set(opt).save()

}
</script>


<template>
  <div v-if="dates != null" class="container">
    <Button as="router-link" to="/admin" icon="pi pi-home" label="retour à la page admin" class="return"/>
      <div id="report-container">
      <h1> Rapport d'ouverture</h1>
      <p> Début: {{ dates[0].toLocaleString('FR-fr') }}</p>
      <p> Fin: {{ dates[1].toLocaleString('FR-fr') }}</p>
      <h3> Récap des commandes : </h3>
      <DataTable v-if="report"
        :value="report" class="report-table">
        <Column :field="(e: ReportItem)=> e.item_name" header="Article"></Column>
        <Column :field="(e: ReportItem) => e.quantity" header="Qtt"></Column>
        <Column :field="(e: ReportItem) => `${e.tva*100}%`" header="TVA"></Column>
        <Column :field="(e: ReportItem) => f_price(e.subtotal_ht)" header="Prix HT"></Column>
        <Column :field="(e: ReportItem) => f_price(e.subtotal_ttc)" header="Prix TTC"></Column>
        <ColumnGroup type="footer">
          <Row>
            <Column/>
            <Column footer="Total :" :colspan="2" footerStyle="text-align:right" />
            <Column :footer="f_price(report.map(e=>e.subtotal_ht).reduce((a,b)=>a+b, 0))" />
            <Column :footer="f_price(report.map(e=>e.subtotal_ttc).reduce((a,b)=>a+b, 0))" />
          </Row>
        </ColumnGroup>
      </DataTable>
      <p v-if="report && report.length == 0" class="no-order">Aucune commande trouvée durant cette période !</p>
    </div>
    <Button label="Télécharger en PDF" :disabled="!report" 
      @click="report?exportToPDF():{}" class="download-pdf" size="large" icon="pi pi-file-pdf"></Button>
  </div>
  <p v-else>Dates invalides</p>
</template>

<style scoped>
.return {
 text-decoration: none; 
}

.container{
  padding: 10px;
}
.no-order{
  text-align: center;
  margin: 20px 0;
}
.download-pdf{
  margin-top: 30px;
}
</style>

<style>
.report-table td, .report-table th{
  padding-left: 0 !important;
  padding-right: 0 !important;
  text-align: center !important;
}

.report-table .p-datatable-column-header-content{
  justify-content: center !important;
}
</style>
