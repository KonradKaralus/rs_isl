use std::path::PathBuf;

use vtkio::{
    model::{
        Attribute, Attributes, DataArray, DataSet, ElementType, Extent, StructuredGridPiece,
        Version,
    },
    Vtk,
};

pub struct VtkWriter {
    idx: usize,
    path: PathBuf,
}

impl VtkWriter {
    pub fn new(path: PathBuf) -> Self {
        Self { idx: 0, path }
    }

    pub fn write_step(&mut self, data: Vec<Vec<f32>>) {
        let dim = (data.len() as u32, data[0].len() as u32);
        let mut points = Vec::with_capacity((dim.0 * dim.1) as usize);
        let mut out_data = Vec::with_capacity((dim.0 * dim.1) as usize);

        for x in 0..dim.0 {
            for y in 0..dim.1 {
                points.append(&mut vec![y as f32, x as f32, 0f32]);
                out_data.push(data[x as usize][y as usize]);
            }
        }

        let out = Vtk {
            version: Version::new((1, 0)),
            byte_order: vtkio::model::ByteOrder::BigEndian,
            title: String::from("output"),
            file_path: None,
            data: DataSet::inline(StructuredGridPiece {
                extent: Extent::Dims([dim.1, dim.0, 1]),
                points: points.into(),
                data: Attributes {
                    point: vec![Attribute::DataArray(DataArray {
                        name: String::from("ptval"),
                        elem: ElementType::Scalars {
                            num_comp: 1,
                            lookup_table: None,
                        },
                        data: out_data.into(),
                    })],
                    cell: vec![],
                },
            }),
        };

        let mut out_path = self.path.clone();
        out_path.push(format!("ISL{:?}.vtk", self.idx));

        out.export(out_path).unwrap();

        self.idx += 1;
    }
}
