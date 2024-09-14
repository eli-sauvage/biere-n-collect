<script setup lang="ts">
import html2pdf from "html2pdf.js"
import type { Ref } from 'vue';
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import { get_report } from './scripts/api/admin/reports';
import Button from "primevue/button";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Row from 'primevue/row';
import ColumnGroup from 'primevue/columngroup';
import { get_stock } from './scripts/api/products';
import { f_price } from './scripts/utils';
const route = useRoute()

type VariationName = string;
type ProductName = string;
type ReportVariation = {
  product_name: ProductName,
  variation_name: VariationName,
  order_quantity: number,
  price_ht: number,
  tva: number,
  price_ttc: number
};
type UniqueVariation = Map<[ProductName, VariationName], ReportVariation>;

type Report = {
  unique_variation: UniqueVariation,
  begin: Date,
  end: Date
};
let report: Ref<Report|null> = ref(null);

(async ()=>{
  if(typeof route.query.begin == "string" && typeof route.query.end == "string"){
    let begin_raw = parseInt(route.query.begin);
    let end_raw = parseInt(route.query.end);
    if(isNaN(begin_raw) || isNaN(end_raw)) return

    let begin = new Date(begin_raw);
    let end = new Date(end_raw);

    let orders = await get_report(begin, end);
    console.log(orders)
    let all_variations = (await get_stock()).map(e=>e.variations).flat()
    let unique_variation: UniqueVariation = new Map();

    for(let order of orders){
      for(let order_variation of order.detail){
        let u_var = unique_variation.get([order_variation.product_name, order_variation.variation_name]);
        if(u_var){
          u_var.order_quantity += 1;
        }else{
          console.log(all_variations)
          console.log(order_variation)
          let variation = all_variations.find(e=>e.id == order_variation.variation_id)
          if(!variation) {
            console.log("not found")
            continue
          }

          let report_var: ReportVariation = {
            product_name: order_variation.product_name,
            variation_name: order_variation.variation_name,
            price_ht: variation.price_ht,
            tva: variation.tva,
            price_ttc: variation.price_ttc,
            order_quantity: 1
          }
          unique_variation.set([order_variation.product_name, order_variation.variation_name], report_var)
        }
      };
    };
    report.value = {unique_variation:unique_variation, begin: begin, end: end}
    console.log(report.value)
  }
})();
function exportToPDF(report: Report){
  const container = document.querySelector("#report-container")
  const fmt = new Intl.DateTimeFormat('fr-FR', {
    day: "2-digit",
    year: "numeric",
    month: "short",
    hour: "2-digit",
    minute: "2-digit"
  });
  const begin_fmt = fmt.format(report.begin).replace(":", "h").replace(/ /g, "").replace(",", "_");
  const end_fmt = fmt.format(report.end).replace(":", "h").replace(/ /g, "").replace(",", "_");
  const opt = {
    margin: 10,
    filename: `rapport-${begin_fmt}-${end_fmt}.pdf`,
    jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' }
  };
  html2pdf().from(container).set(opt).save()

}
</script>


<template>
  <div class="container">
    <div id="report-container">
      <h1> Rapport d'ouverture</h1>
      <p> Début: {{ report?.begin.toLocaleString('FR-fr') }}</p>
      <p> Fin: {{ report?.end.toLocaleString('FR-fr') }}</p>
      <DataTable v-if="report && report.unique_variation.values.length"
        :value="Array.from(report.unique_variation.values())" class="report-table">
        <Column :field="(e: ReportVariation)=>`${e.product_name}: ${e.variation_name}`" header="Article"></Column>
        <Column :field="(e: ReportVariation) => e.order_quantity" header="Qtt"></Column>
        <Column :field="(e: ReportVariation) => `${e.tva*100}%`" header="TVA"></Column>
        <Column :field="(e: ReportVariation) => f_price(e.price_ht)" header="Prix HT"></Column>
        <Column :field="(e: ReportVariation) => f_price(e.price_ttc)" header="Prix TTC"></Column>
        <ColumnGroup type="footer">
          <Row>
            <Column/>
            <Column footer="Total :" :colspan="2" footerStyle="text-align:right" />
            <Column :footer="f_price(Array.from(report.unique_variation.values()).map(e=>e.price_ht).reduce((a, b)=>a+b))" />
            <Column :footer="f_price(Array.from(report.unique_variation.values()).map(e=>e.price_ttc).reduce((a, b)=>a+b))" />
          </Row>
        </ColumnGroup>
      </DataTable>
      <p v-else class="no-order">Aucune commande trouvée durant cette période !</p>
    </div>
    <Button label="Télécharger en PDF" :disabled="!(report && report.unique_variation.values.length)" 
      @click="report?exportToPDF(report):{}" class="download-pdf" size="large" icon="pi pi-file-pdf"></Button>
  </div>
</template>

<style scoped>
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
