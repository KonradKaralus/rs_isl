use std::{marker::PhantomData, path::PathBuf};

use vtkio::{
    model::{
        Attribute, Attributes, DataArray, DataSet, ElementType, Extent, StructuredGridPiece,
        Version,
    },
    Vtk,
};

use crate::VtkOutput;

pub struct VtkWriter<T>
where
    T: VtkOutput,
{
    idx: usize,
    path: PathBuf,
    r_type: PhantomData<T>,
    rows: Vec<String>,
}

impl<T> VtkWriter<T>
where
    T: VtkOutput,
{
    pub fn new(path: PathBuf, rows: Vec<String>) -> Self {
        Self {
            idx: 0,
            path,
            r_type: PhantomData,
            rows,
        }
    }

    pub fn write_step(&mut self, data: Vec<Vec<T>>) {
        let num_values = self.rows.len();
        let dim = (data.len() as u32, data[0].len() as u32);
        let mut points = Vec::with_capacity((dim.0 * dim.1) as usize);
        let mut out_data = vec![Vec::with_capacity((dim.0 * dim.1) as usize); num_values];

        for x in 0..dim.0 {
            for y in 0..dim.1 {
                points.append(&mut vec![y as f32, x as f32, 0f32]);
                let cv = data[x as usize][y as usize].cellvalue();
                if cv.len() != self.rows.len() {
                    panic!("found irregular length of values when creating vtk output");
                }
                for i in 0..num_values {
                    out_data[i].push(cv[i]);
                }
            }
        }

        let mut point_data = vec![];

        for (idx, line) in out_data.into_iter().enumerate() {
            point_data.push(Attribute::DataArray(DataArray {
                name: self.rows[idx].clone(),
                elem: ElementType::Scalars {
                    num_comp: 1,
                    lookup_table: None,
                },
                data: line.into(),
            }));
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
                    point: point_data,
                    cell: vec![],
                },
            }),
        };

        let mut out_path = self.path.clone();
        out_path.push(format!("ISL{:?}.vtk", self.idx));

        out.export_ascii(out_path).unwrap();

        self.idx += 1;
    }
}
