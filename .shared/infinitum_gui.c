#include <stdlib.h>
#include <string.h>
#include "gui.h"

void gui_element_init(struct gui_element_t *element, uint32_t kind, gui_id_t id) {
    if (element == NULL) {
        return;
    }
    memset(element, 0, sizeof(*element));
    element->abi_version = CANOR_GUI_ABI_VERSION;
    element->kind = kind;
    element->id = id;
    element->flags = GUI_FLAG_ENABLED;
    element->layout.visibility = GUI_VISIBILITY_VISIBLE;
}

void gui_document_init(struct gui_document_t *document) {
    if (document == NULL) {
        return;
    }
    memset(document, 0, sizeof(*document));
    document->abi_version = CANOR_GUI_ABI_VERSION;
}

int gui_document_reserve(struct gui_document_t *document, uint64_t element_count) {
    struct gui_element_t *elements;
    uint64_t i;
    if (document == NULL) {
        return -1;
    }
    if (element_count == 0u) {
        free(document->elements);
        document->elements = NULL;
        document->element_count = 0u;
        return 0;
    }
    if (element_count > (SIZE_MAX / sizeof(struct gui_element_t))) {
        return -2;
    }
    elements = (struct gui_element_t *)calloc((size_t)element_count, sizeof(struct gui_element_t));
    if (elements == NULL) {
        return -3;
    }
    free(document->elements);
    document->elements = elements;
    document->element_count = element_count;
    for (i = 0u; i < element_count; ++i) {
        gui_element_init(&document->elements[i], GUI_ELEMENT_NONE, 0u);
    }
    return 0;
}

void gui_document_free(struct gui_document_t *document) {
    if (document == NULL) {
        return;
    }
    free(document->elements);
    document->elements = NULL;
    document->element_count = 0u;
}
