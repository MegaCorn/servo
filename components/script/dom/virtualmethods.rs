/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use html5ever::LocalName;
use style::attr::AttrValue;

use crate::dom::attr::Attr;
use crate::dom::bindings::inheritance::{
    Castable, DocumentFragmentTypeId, ElementTypeId, HTMLElementTypeId, HTMLMediaElementTypeId,
    NodeTypeId, SVGElementTypeId, SVGGraphicsElementTypeId,
};
use crate::dom::bindings::str::DOMString;
use crate::dom::document::Document;
use crate::dom::documentfragment::DocumentFragment;
use crate::dom::element::{AttributeMutation, Element};
use crate::dom::event::Event;
use crate::dom::htmlanchorelement::HTMLAnchorElement;
use crate::dom::htmlareaelement::HTMLAreaElement;
use crate::dom::htmlbaseelement::HTMLBaseElement;
use crate::dom::htmlbodyelement::HTMLBodyElement;
use crate::dom::htmlbuttonelement::HTMLButtonElement;
use crate::dom::htmlcanvaselement::HTMLCanvasElement;
use crate::dom::htmldetailselement::HTMLDetailsElement;
use crate::dom::htmlelement::HTMLElement;
use crate::dom::htmlfieldsetelement::HTMLFieldSetElement;
use crate::dom::htmlfontelement::HTMLFontElement;
use crate::dom::htmlformelement::HTMLFormElement;
use crate::dom::htmlheadelement::HTMLHeadElement;
use crate::dom::htmlhrelement::HTMLHRElement;
use crate::dom::htmliframeelement::HTMLIFrameElement;
use crate::dom::htmlimageelement::HTMLImageElement;
use crate::dom::htmlinputelement::HTMLInputElement;
use crate::dom::htmllabelelement::HTMLLabelElement;
use crate::dom::htmllielement::HTMLLIElement;
use crate::dom::htmllinkelement::HTMLLinkElement;
use crate::dom::htmlmediaelement::HTMLMediaElement;
use crate::dom::htmlmetaelement::HTMLMetaElement;
use crate::dom::htmlobjectelement::HTMLObjectElement;
use crate::dom::htmloptgroupelement::HTMLOptGroupElement;
use crate::dom::htmloptionelement::HTMLOptionElement;
use crate::dom::htmloutputelement::HTMLOutputElement;
use crate::dom::htmlpreelement::HTMLPreElement;
use crate::dom::htmlscriptelement::HTMLScriptElement;
use crate::dom::htmlselectelement::HTMLSelectElement;
use crate::dom::htmlslotelement::HTMLSlotElement;
use crate::dom::htmlsourceelement::HTMLSourceElement;
use crate::dom::htmlstyleelement::HTMLStyleElement;
use crate::dom::htmltablecellelement::HTMLTableCellElement;
use crate::dom::htmltablecolelement::HTMLTableColElement;
use crate::dom::htmltableelement::HTMLTableElement;
use crate::dom::htmltablerowelement::HTMLTableRowElement;
use crate::dom::htmltablesectionelement::HTMLTableSectionElement;
use crate::dom::htmltemplateelement::HTMLTemplateElement;
use crate::dom::htmltextareaelement::HTMLTextAreaElement;
use crate::dom::htmltitleelement::HTMLTitleElement;
use crate::dom::htmlvideoelement::HTMLVideoElement;
use crate::dom::node::{BindContext, ChildrenMutation, CloneChildrenFlag, Node, UnbindContext};
use crate::dom::shadowroot::ShadowRoot;
use crate::dom::svgelement::SVGElement;
use crate::dom::svgsvgelement::SVGSVGElement;

use crate::dom::characterdata::CharacterData;
use crate::dom::documenttype::DocumentType;
use crate::dom::htmlbrelement::HTMLBRElement;
use crate::dom::htmldlistelement::HTMLDListElement;
use crate::dom::htmldataelement::HTMLDataElement;
use crate::dom::htmldatalistelement::HTMLDataListElement;
use crate::dom::htmldialogelement::HTMLDialogElement;
use crate::dom::htmldirectoryelement::HTMLDirectoryElement;
use crate::dom::htmldivelement::HTMLDivElement;
use crate::dom::htmlembedelement::HTMLEmbedElement;
use crate::dom::htmlframeelement::HTMLFrameElement;
use crate::dom::htmlframesetelement::HTMLFrameSetElement;
use crate::dom::htmlheadingelement::HTMLHeadingElement;
use crate::dom::htmlhtmlelement::HTMLHtmlElement;
use crate::dom::htmllegendelement::HTMLLegendElement;
use crate::dom::htmlmapelement::HTMLMapElement;
use crate::dom::htmlmenuelement::HTMLMenuElement;
use crate::dom::htmlmeterelement::HTMLMeterElement;
use crate::dom::htmlmodelement::HTMLModElement;
use crate::dom::htmlolistelement::HTMLOListElement;
use crate::dom::htmlparagraphelement::HTMLParagraphElement;
use crate::dom::htmlparamelement::HTMLParamElement;
use crate::dom::htmlpictureelement::HTMLPictureElement;
use crate::dom::htmlprogresselement::HTMLProgressElement;
use crate::dom::htmlquoteelement::HTMLQuoteElement;
use crate::dom::htmltablecaptionelement::HTMLTableCaptionElement;
use crate::dom::htmltimeelement::HTMLTimeElement;
use crate::dom::htmltrackelement::HTMLTrackElement;
use crate::dom::htmlulistelement::HTMLUListElement;
use crate::dom::htmlunknownelement::HTMLUnknownElement;
use crate::dom::htmlspanelement::HTMLSpanElement;

/// Trait to allow DOM nodes to opt-in to overriding (or adding to) common
/// behaviours. Replicates the effect of C++ virtual methods.
pub(crate) trait VirtualMethods {
    /// Returns self as the superclass of the implementation for this trait,
    /// if any.
    fn super_type(&self) -> Option<&dyn VirtualMethods>;

    /// Called when attributes of a node are mutated.
    /// <https://dom.spec.whatwg.org/#attribute-is-set>
    /// <https://dom.spec.whatwg.org/#attribute-is-removed>
    fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
        if let Some(s) = self.super_type() {
            s.attribute_mutated(attr, mutation);
        }
    }

    /// Returns `true` if given attribute `attr` affects style of the
    /// given element.
    fn attribute_affects_presentational_hints(&self, attr: &Attr) -> bool {
        match self.super_type() {
            Some(s) => s.attribute_affects_presentational_hints(attr),
            None => false,
        }
    }

    /// Returns the right AttrValue variant for the attribute with name `name`
    /// on this element.
    fn parse_plain_attribute(&self, name: &LocalName, value: DOMString) -> AttrValue {
        match self.super_type() {
            Some(s) => s.parse_plain_attribute(name, value),
            _ => AttrValue::String(value.into()),
        }
    }

    /// Invoked during a DOM tree mutation after a node becomes connected, once all
    /// related DOM tree mutations have been applied.
    /// <https://dom.spec.whatwg.org/#concept-node-post-connection-ext>
    fn post_connection_steps(&self) {
        if let Some(s) = self.super_type() {
            s.post_connection_steps();
        }
    }

    /// Called when a Node is appended to a tree, where 'tree_connected' indicates
    /// whether the tree is part of a Document.
    fn bind_to_tree(&self, context: &BindContext) {
        if let Some(s) = self.super_type() {
            s.bind_to_tree(context);
        }
    }

    /// Called when a Node is removed from a tree, where 'tree_connected'
    /// indicates whether the tree is part of a Document.
    /// Implements removing steps:
    /// <https://dom.spec.whatwg.org/#concept-node-remove-ext>
    fn unbind_from_tree(&self, context: &UnbindContext) {
        if let Some(s) = self.super_type() {
            s.unbind_from_tree(context);
        }
    }

    /// Called on the parent when its children are changed.
    fn children_changed(&self, mutation: &ChildrenMutation) {
        if let Some(s) = self.super_type() {
            s.children_changed(mutation);
        }
    }

    /// Called during event dispatch after the bubbling phase completes.
    fn handle_event(&self, event: &Event) {
        if let Some(s) = self.super_type() {
            s.handle_event(event);
        }
    }

    /// <https://dom.spec.whatwg.org/#concept-node-adopt-ext>
    fn adopting_steps(&self, old_doc: &Document) {
        if let Some(s) = self.super_type() {
            s.adopting_steps(old_doc);
        }
    }

    /// <https://dom.spec.whatwg.org/#concept-node-clone-ext>
    fn cloning_steps(
        &self,
        copy: &Node,
        maybe_doc: Option<&Document>,
        clone_children: CloneChildrenFlag,
    ) {
        if let Some(s) = self.super_type() {
            s.cloning_steps(copy, maybe_doc, clone_children);
        }
    }

    /// Called on an element when it is popped off the stack of open elements
    /// of a parser.
    fn pop(&self) {
        if let Some(s) = self.super_type() {
            s.pop();
        }
    }
}

pub fn node_downcast_template1<'s>(node: &Node, scope: &mut v8::HandleScope<'s>) -> v8::Local<'s, v8::ObjectTemplate> {
    match node.type_id() {
        NodeTypeId::CharacterData(CharacterDataTypeId) => {
            let node_ = node.downcast::<CharacterData>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::DocumentType => {
            let node_ = node.downcast::<DocumentType>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLElement)) => {
            let node_ = node.downcast::<HTMLElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAnchorElement)) => {
            let node_ = node.downcast::<HTMLAnchorElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAreaElement)) => {
            let node_ = node.downcast::<HTMLAreaElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBRElement)) => {
            let node_ = node.downcast::<HTMLBRElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBaseElement)) => {
            let node_ = node.downcast::<HTMLBaseElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBodyElement)) => {
            let node_ = node.downcast::<HTMLBodyElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
            let node_ = node.downcast::<HTMLButtonElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLCanvasElement)) => {
            let node_ = node.downcast::<HTMLCanvasElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDListElement)) => {
            let node_ = node.downcast::<HTMLDListElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDataElement)) => {
            let node_ = node.downcast::<HTMLDataElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDataListElement)) => {
            let node_ = node.downcast::<HTMLDataListElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDetailsElement)) => {
            let node_ = node.downcast::<HTMLDetailsElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDialogElement)) => {
            let node_ = node.downcast::<HTMLDialogElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDirectoryElement)) => {
            let node_ = node.downcast::<HTMLDirectoryElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDivElement)) => {
            let node_ = node.downcast::<HTMLDivElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLEmbedElement)) => {
            let node_ = node.downcast::<HTMLEmbedElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFieldSetElement)) => {
            let node_ = node.downcast::<HTMLFieldSetElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFontElement)) => {
            let node_ = node.downcast::<HTMLFontElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFormElement)) => {
            let node_ = node.downcast::<HTMLFormElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFrameElement)) => {
            let node_ = node.downcast::<HTMLFrameElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFrameSetElement)) => {
            let node_ = node.downcast::<HTMLFrameSetElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHRElement)) => {
            let node_ = node.downcast::<HTMLHRElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHeadElement)) => {
            let node_ = node.downcast::<HTMLHeadElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHeadingElement)) => {
            let node_ = node.downcast::<HTMLHeadingElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHtmlElement)) => {
            let node_ = node.downcast::<HTMLHtmlElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLIFrameElement)) => {
            let node_ = node.downcast::<HTMLIFrameElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLImageElement)) => {
            let node_ = node.downcast::<HTMLImageElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
            let node_ = node.downcast::<HTMLInputElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLIElement)) => {
            let node_ = node.downcast::<HTMLLIElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLabelElement)) => {
            let node_ = node.downcast::<HTMLLabelElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLegendElement)) => {
            let node_ = node.downcast::<HTMLLegendElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLinkElement)) => {
            let node_ = node.downcast::<HTMLLinkElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMapElement)) => {
            let node_ = node.downcast::<HTMLMapElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMediaElement(HTMLMediaElementTypeId))) => {
            let node_ = node.downcast::<HTMLMediaElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMenuElement)) => {
            let node_ = node.downcast::<HTMLMenuElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMetaElement)) => {
            let node_ = node.downcast::<HTMLMetaElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMeterElement)) => {
            let node_ = node.downcast::<HTMLMeterElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLModElement)) => {
            let node_ = node.downcast::<HTMLModElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOListElement)) => {
            let node_ = node.downcast::<HTMLOListElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
            let node_ = node.downcast::<HTMLObjectElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOptGroupElement)) => {
            let node_ = node.downcast::<HTMLOptGroupElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOptionElement)) => {
            let node_ = node.downcast::<HTMLOptionElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOutputElement)) => {
            let node_ = node.downcast::<HTMLOutputElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLParagraphElement)) => {
            let node_ = node.downcast::<HTMLParagraphElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLParamElement)) => {
            let node_ = node.downcast::<HTMLParamElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLPictureElement)) => {
            let node_ = node.downcast::<HTMLPictureElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLPreElement)) => {
            let node_ = node.downcast::<HTMLPreElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLProgressElement)) => {
            let node_ = node.downcast::<HTMLProgressElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLQuoteElement)) => {
            let node_ = node.downcast::<HTMLQuoteElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLScriptElement)) => {
            let node_ = node.downcast::<HTMLScriptElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
            let node_ = node.downcast::<HTMLSelectElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSourceElement)) => {
            let node_ = node.downcast::<HTMLSourceElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSpanElement)) => {
            let node_ = node.downcast::<HTMLSpanElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLStyleElement)) => {
            let node_ = node.downcast::<HTMLStyleElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableCaptionElement)) => {
            let node_ = node.downcast::<HTMLTableCaptionElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableCellElement)) => {
            let node_ = node.downcast::<HTMLTableCellElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableColElement)) => {
            let node_ = node.downcast::<HTMLTableColElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableElement)) => {
            let node_ = node.downcast::<HTMLTableElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableRowElement)) => {
            let node_ = node.downcast::<HTMLTableRowElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableSectionElement)) => {
            let node_ = node.downcast::<HTMLTableSectionElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTemplateElement)) => {
            let node_ = node.downcast::<HTMLTemplateElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
            let node_ = node.downcast::<HTMLTextAreaElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTimeElement)) => {
            let node_ = node.downcast::<HTMLTimeElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTitleElement)) => {
            let node_ = node.downcast::<HTMLTitleElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTrackElement)) => {
            let node_ = node.downcast::<HTMLTrackElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLUListElement)) => {
            let node_ = node.downcast::<HTMLUListElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLUnknownElement)) => {
            let node_ = node.downcast::<HTMLUnknownElement>().unwrap();
            node_.new_template(scope)
        },
        _ => {
            log::error!("====================== node_downcast_template {:?} fail ======================", node.type_id());
            node.new_template(scope)
        },
    }
}

pub fn node_downcast_template<'s>(element: &Element, scope: &mut v8::HandleScope<'s>) -> v8::Local<'s, v8::ObjectTemplate> {
    let node = element.upcast::<Node>();
    match node.type_id() {
        NodeTypeId::CharacterData(CharacterDataTypeId) => {
            let node_ = node.downcast::<CharacterData>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::DocumentType => {
            let node_ = node.downcast::<Document>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLElement)) => {
            let node_ = node.downcast::<HTMLElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAnchorElement)) => {
            let node_ = node.downcast::<HTMLAnchorElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAreaElement)) => {
            let node_ = node.downcast::<HTMLAreaElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBRElement)) => {
            let node_ = node.downcast::<HTMLBRElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBaseElement)) => {
            let node_ = node.downcast::<HTMLBaseElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBodyElement)) => {
            let node_ = node.downcast::<HTMLBodyElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
            let node_ = node.downcast::<HTMLButtonElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLCanvasElement)) => {
            let node_ = node.downcast::<HTMLCanvasElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDListElement)) => {
            let node_ = node.downcast::<HTMLDListElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDataElement)) => {
            let node_ = node.downcast::<HTMLDataElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDataListElement)) => {
            let node_ = node.downcast::<HTMLDataListElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDetailsElement)) => {
            let node_ = node.downcast::<HTMLDetailsElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDialogElement)) => {
            let node_ = node.downcast::<HTMLDialogElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDirectoryElement)) => {
            let node_ = node.downcast::<HTMLDirectoryElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDivElement)) => {
            let node_ = node.downcast::<HTMLDivElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLEmbedElement)) => {
            let node_ = node.downcast::<HTMLEmbedElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFieldSetElement)) => {
            let node_ = node.downcast::<HTMLFieldSetElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFontElement)) => {
            let node_ = node.downcast::<HTMLFontElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFormElement)) => {
            let node_ = node.downcast::<HTMLFormElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFrameElement)) => {
            let node_ = node.downcast::<HTMLFrameElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFrameSetElement)) => {
            let node_ = node.downcast::<HTMLFrameSetElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHRElement)) => {
            let node_ = node.downcast::<HTMLHRElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHeadElement)) => {
            let node_ = node.downcast::<HTMLHeadElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHeadingElement)) => {
            let node_ = node.downcast::<HTMLHeadingElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHtmlElement)) => {
            let node_ = node.downcast::<HTMLHtmlElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLIFrameElement)) => {
            let node_ = node.downcast::<HTMLIFrameElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLImageElement)) => {
            let node_ = node.downcast::<HTMLImageElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
            let node_ = node.downcast::<HTMLInputElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLIElement)) => {
            let node_ = node.downcast::<HTMLLIElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLabelElement)) => {
            let node_ = node.downcast::<HTMLLabelElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLegendElement)) => {
            let node_ = node.downcast::<HTMLLegendElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLinkElement)) => {
            let node_ = node.downcast::<HTMLLinkElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMapElement)) => {
            let node_ = node.downcast::<HTMLMapElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMediaElement(HTMLMediaElementTypeId))) => {
            let node_ = node.downcast::<HTMLMediaElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMenuElement)) => {
            let node_ = node.downcast::<HTMLMenuElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMetaElement)) => {
            let node_ = node.downcast::<HTMLMetaElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMeterElement)) => {
            let node_ = node.downcast::<HTMLMeterElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLModElement)) => {
            let node_ = node.downcast::<HTMLModElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOListElement)) => {
            let node_ = node.downcast::<HTMLOListElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
            let node_ = node.downcast::<HTMLObjectElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOptGroupElement)) => {
            let node_ = node.downcast::<HTMLOptGroupElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOptionElement)) => {
            let node_ = node.downcast::<HTMLOptionElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOutputElement)) => {
            let node_ = node.downcast::<HTMLOutputElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLParagraphElement)) => {
            let node_ = node.downcast::<HTMLParagraphElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLParamElement)) => {
            let node_ = node.downcast::<HTMLParamElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLPictureElement)) => {
            let node_ = node.downcast::<HTMLPictureElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLPreElement)) => {
            let node_ = node.downcast::<HTMLPreElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLProgressElement)) => {
            let node_ = node.downcast::<HTMLProgressElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLQuoteElement)) => {
            let node_ = node.downcast::<HTMLQuoteElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLScriptElement)) => {
            let node_ = node.downcast::<HTMLScriptElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
            let node_ = node.downcast::<HTMLSelectElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSourceElement)) => {
            let node_ = node.downcast::<HTMLSourceElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSpanElement)) => {
            let node_ = node.downcast::<HTMLSpanElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLStyleElement)) => {
            let node_ = node.downcast::<HTMLStyleElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableCaptionElement)) => {
            let node_ = node.downcast::<HTMLTableCaptionElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableCellElement)) => {
            let node_ = node.downcast::<HTMLTableCellElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableColElement)) => {
            let node_ = node.downcast::<HTMLTableColElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableElement)) => {
            let node_ = node.downcast::<HTMLTableElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableRowElement)) => {
            let node_ = node.downcast::<HTMLTableRowElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableSectionElement)) => {
            let node_ = node.downcast::<HTMLTableSectionElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTemplateElement)) => {
            let node_ = node.downcast::<HTMLTemplateElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
            let node_ = node.downcast::<HTMLTextAreaElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTimeElement)) => {
            let node_ = node.downcast::<HTMLTimeElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTitleElement)) => {
            let node_ = node.downcast::<HTMLTitleElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTrackElement)) => {
            let node_ = node.downcast::<HTMLTrackElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLUListElement)) => {
            let node_ = node.downcast::<HTMLUListElement>().unwrap();
            node_.new_template(scope)
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLUnknownElement)) => {
            let node_ = node.downcast::<HTMLUnknownElement>().unwrap();
            node_.new_template(scope)
        },
        _ => {
            log::error!("====================== node_downcast_template {:?} fail ======================", node.type_id());
            element.new_template(scope)
        },
    }
}

/// Obtain a VirtualMethods instance for a given Node-derived object. Any
/// method call on the trait object will invoke the corresponding method on the
/// concrete type, propagating up the parent hierarchy unless otherwise
/// interrupted.
pub(crate) fn vtable_for(node: &Node) -> &dyn VirtualMethods {
    match node.type_id() {
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAnchorElement)) => {
            node.downcast::<HTMLAnchorElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAreaElement)) => {
            node.downcast::<HTMLAreaElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBaseElement)) => {
            node.downcast::<HTMLBaseElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBodyElement)) => {
            node.downcast::<HTMLBodyElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
            node.downcast::<HTMLButtonElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLCanvasElement)) => {
            node.downcast::<HTMLCanvasElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDetailsElement)) => {
            node.downcast::<HTMLDetailsElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFieldSetElement)) => {
            node.downcast::<HTMLFieldSetElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFontElement)) => {
            node.downcast::<HTMLFontElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFormElement)) => {
            node.downcast::<HTMLFormElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHeadElement)) => {
            node.downcast::<HTMLHeadElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHRElement)) => {
            node.downcast::<HTMLHRElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLImageElement)) => {
            node.downcast::<HTMLImageElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLIFrameElement)) => {
            node.downcast::<HTMLIFrameElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
            node.downcast::<HTMLInputElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLabelElement)) => {
            node.downcast::<HTMLLabelElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLIElement)) => {
            node.downcast::<HTMLLIElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLinkElement)) => {
            node.downcast::<HTMLLinkElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMediaElement(
            media_el,
        ))) => match media_el {
            HTMLMediaElementTypeId::HTMLVideoElement => {
                node.downcast::<HTMLVideoElement>().unwrap() as &dyn VirtualMethods
            },
            _ => node.downcast::<HTMLMediaElement>().unwrap() as &dyn VirtualMethods,
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMetaElement)) => {
            node.downcast::<HTMLMetaElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
            node.downcast::<HTMLObjectElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOptGroupElement)) => {
            node.downcast::<HTMLOptGroupElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOptionElement)) => {
            node.downcast::<HTMLOptionElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOutputElement)) => {
            node.downcast::<HTMLOutputElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLPreElement)) => {
            node.downcast::<HTMLPreElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLScriptElement)) => {
            node.downcast::<HTMLScriptElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
            node.downcast::<HTMLSelectElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSourceElement)) => {
            node.downcast::<HTMLSourceElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSlotElement)) => {
            node.downcast::<HTMLSlotElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLStyleElement)) => {
            node.downcast::<HTMLStyleElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableElement)) => {
            node.downcast::<HTMLTableElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(
            HTMLElementTypeId::HTMLTableCellElement,
        )) => node.downcast::<HTMLTableCellElement>().unwrap() as &dyn VirtualMethods,
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableColElement)) => {
            node.downcast::<HTMLTableColElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableRowElement)) => {
            node.downcast::<HTMLTableRowElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(
            HTMLElementTypeId::HTMLTableSectionElement,
        )) => node.downcast::<HTMLTableSectionElement>().unwrap() as &dyn VirtualMethods,
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTemplateElement)) => {
            node.downcast::<HTMLTemplateElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
            node.downcast::<HTMLTextAreaElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTitleElement)) => {
            node.downcast::<HTMLTitleElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::SVGElement(SVGElementTypeId::SVGGraphicsElement(
            SVGGraphicsElementTypeId::SVGSVGElement,
        ))) => node.downcast::<SVGSVGElement>().unwrap() as &dyn VirtualMethods,
        NodeTypeId::Element(ElementTypeId::SVGElement(SVGElementTypeId::SVGElement)) => {
            node.downcast::<SVGElement>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(ElementTypeId::Element) => {
            node.downcast::<Element>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::Element(_) => node.downcast::<HTMLElement>().unwrap() as &dyn VirtualMethods,
        NodeTypeId::DocumentFragment(DocumentFragmentTypeId::ShadowRoot) => {
            node.downcast::<ShadowRoot>().unwrap() as &dyn VirtualMethods
        },
        NodeTypeId::DocumentFragment(_) => {
            node.downcast::<DocumentFragment>().unwrap() as &dyn VirtualMethods
        },
        _ => node as &dyn VirtualMethods,
    }
}
