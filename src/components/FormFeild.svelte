<script>
  /**
     * @type {any}
     */
   export let field
  /**
     * @type {any}
     */
   export let value

   /**
     * @type {String}
     */
   export let typeName = 'text'
  
   /**
    * @param {HTMLElement} node 
   */
   function typeAction(node) {
      // @ts-ignore
      node.type = typeName;
   }

</script>

<div class="form-col">
  <label for={field}>{field}</label>
    <input id={field} use:typeAction bind:value>
</div>


<style>
    /* Form sections */
    .form-col {
        width: 80%;
        margin: 0 auto;
    }

    label {
        text-align: left;
        display: block;
        margin-bottom: 6px;
    }

    input {
        width: 100%;
        padding: 10px;
        border-radius: 5px;
        border: none;
        background: #3E3E3E;
        color: #fff;
    }
</style>