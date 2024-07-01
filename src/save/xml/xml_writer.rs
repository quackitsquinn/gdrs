use std::io::Write;

use xml::{writer::XmlEvent, EventWriter};

use super::RawLevel;

pub fn xml_from_level_list<T>(levels: T) -> String
where
    T: AsRef<[RawLevel]>,
{
    let levels = levels.as_ref();
    let mut xml = EventWriter::new(Vec::new());

    generate_xml(&mut xml, levels);

    String::from_utf8(xml.into_inner()).unwrap()
}

fn generate_xml<T>(w: &mut EventWriter<T>, levels: &[RawLevel])
where
    T: Write,
{
    // Write the header
    generate_header(w);

    // Write the levels
    for (i, level) in levels.iter().enumerate() {
        w.write(XmlEvent::start_element("k")).unwrap();
        w.write(XmlEvent::characters(&format!("k_{}", i))).unwrap();
        w.write(XmlEvent::end_element()).unwrap();
        w.write(XmlEvent::start_element("d")).unwrap();
        level.write_xml(w);
        w.write(XmlEvent::end_element()).unwrap();
    }

    // Write the footer
    generate_footer(w);
}

fn generate_header<T>(w: &mut EventWriter<T>)
where
    T: Write,
{
    w.write(
        XmlEvent::start_element("plist")
            .attr("version", "1.0")
            .attr("gjver", "2.0"),
    )
    .unwrap();
    w.write(XmlEvent::start_element("dict")).unwrap();
    w.write(XmlEvent::start_element("k")).unwrap();
    w.write(XmlEvent::characters("LLM_01")).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
    w.write(XmlEvent::start_element("d")).unwrap();
    w.write(XmlEvent::start_element("k")).unwrap();
    w.write(XmlEvent::characters("_isArr")).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
    w.write(XmlEvent::start_element("t")).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
}

fn generate_footer<T>(w: &mut EventWriter<T>)
where
    T: Write,
{
    // TODO: figure out what LLM_02 is
    w.write(XmlEvent::end_element()).unwrap();
    w.write(XmlEvent::start_element("k")).unwrap();
    w.write(XmlEvent::characters("LLM_02")).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
    w.write(XmlEvent::start_element("i")).unwrap();
    w.write(XmlEvent::characters("35")).unwrap(); // magic number transplanted from my save file
    w.write(XmlEvent::end_element()).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
}
