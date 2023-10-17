import * as yup from "yup";

const partSchema = yup.object({
  part_number: yup.string().trim().required("Part Number missing from Part"),
  invoice_number: yup
    .number()
    .required("Invoice Number is required for every part"),
  distributor_number: yup
    .number()
    .required("Distributor is required for every part"),
});

const customerSchema = yup.object({
  first_name: yup.string().trim().required("Customer First Name is required"),
  last_name: yup.string().trim().required("Customer Last Name is required"),
  address_1: yup.string().trim().required("Customer Address is required"),
  city: yup.string().trim().required("Customer City is required"),
  zip_code: yup
    .string()
    .trim()
    .matches(/^\d{5}$/, "Zip Code must be a 5 digit number")
    .required("Customer Zip Code is required"),
  email: yup.string().email(),
  phone_number: yup
    .string()
    .matches(
      /^[+]?[(]?[0-9]{3}[)]?[-s. ]?[0-9]{3}[-s. ]?[0-9]{4}$/,
      "Not a valid phone number, try this format: (XXX)-XXX-XXXX"
    )
    .required("Customer Phone Number is required"),
});

const applianceSchema = yup.object({
  product_code: yup
    .string()
    .trim()
    .min(8)
    .max(10)
    .required("Appliance Product Code is required"), // TODO Make regex check for this
  // ^([TD][CR]|FF|SF)[357]00[34578]W[ENG]$
  serial_number: yup
    .string()
    .matches(/^\d{10}$/, "Serial Number must be a 10 digit number")
    .required("Appliance Serial Number is required"),
  model_number: yup
    .string()
    .trim()
    .min(8)
    .required("Appliance Model Number is required"),
  purchase_date: yup.date().required("Appliance Purchase Date is required"),
});

const laborSchema = yup.object({
  date_requested: yup.date().required("Service Request Date is required"),
  date_completed: yup.date().required("Service Completed Date is required"),
  miles_traveled: yup
    .number()
    .min(1)
    .max(999)
    .required("Miles Traveled is required"),
  repair_code: yup
    .number()
    .min(1, "Must select a Repair Code")
    .required("Repair Code is required"),
  defect_code: yup
    .number()
    .min(1, "Must select a Defect Code")
    .required("Defect Code is required"),
  issue_description: yup
    .string()
    .trim()
    // .length(10, "Issue Description is too short, add more details")
    .required("Issue Description is required"),
  service_performed: yup
    .string()
    .trim()
    // .length(10, "Service Performed is too short, add more details")
    .required("Service Performed is required"),
});

export const invoiceSchema = yup
  .object({
    parts: yup.array().of(partSchema),
  })
  .concat(customerSchema)
  .concat(laborSchema)
  .concat(applianceSchema);
