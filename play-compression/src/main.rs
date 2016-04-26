
extern crate brotli;
extern crate flate2;
extern crate lz4;
extern crate snappy;

use std::io::{  BufWriter, BufReader };

use std::io::prelude::*;

use brotli::Decompressor;

use flate2::Compression;
use flate2::read::{DeflateDecoder, GzDecoder};
use flate2::write::{DeflateEncoder, GzEncoder};

use snappy::compress;

use std::fs::File;
use std::path::Path;

// references:
//    https://github.com/fulmicoton/tantivy/
//    http://catchchallenger.first-world.info/wiki/Quick_Benchmark:_Gzip_vs_Bzip2_vs_LZMA_vs_XZ_vs_LZ4_vs_LZO

// kafka supports gzip, snappy and lz4.
//    https://github.com/spicavigo/kafka-rust/tree/master/src/compression
//    https://github.com/apache/kafka/blob/trunk/clients/src/main/java/org/apache/kafka/common/record/CompressionType.java


// Brotli: native decompress in rust
//         https://github.com/ende76/brotli-rs
//         https://github.com/google/brotli/tree/master/dec

// how to use brotli in mac
//     sudo port install brotli
//     bro  --input asset/s3-2006-03-01.normal.json --output asset/s3-2006-03-01.normal.json.brotli
fn decompress_brotli( ) {

    let brotli_stream = std::fs::File::open("asset/s3-2006-03-01.normal.json.brotli").unwrap();

    //let mut decompressed : Vec<u8> = Vec::<u8>::new();
    let mut decompressed : Vec<u8> = Vec::new();
    let _ = Decompressor::new(brotli_stream).read_to_end(&mut decompressed);

    let mut fout = BufWriter::new(File::create(Path::new("asset/s3-2006-03-01.normal.json.brotli.decompressed")).unwrap());
    fout.write(&decompressed).unwrap();
}

// Gzip & Deflate
//   https://github.com/alexcrichton/flate2-rs
pub fn compress_decompress_gzip() {

    {
        // None, Fast, Best, Default
        let jsonfile = std::fs::File::open("asset/s3-2006-03-01.normal.json").unwrap();
        let mut reader = BufReader::new(jsonfile);
        let mut jsoncontent = Vec::new();
        reader.read_to_end(&mut jsoncontent).unwrap();

        let mut encoder = GzEncoder::new(Vec::new(), Compression::Default);
        encoder.write(&jsoncontent).unwrap();
        let compressed = encoder.finish().unwrap();

        let mut fout = BufWriter::new(File::create(Path::new("asset/s3-2006-03-01.normal.json.gz")).unwrap());
        fout.write(&compressed).unwrap();
    }

    {
        let mut vf = Vec::new();
        let jsonfile = std::fs::File::open("asset/s3-2006-03-01.normal.json.gz").unwrap();

        let mut decoder =  GzDecoder::new(jsonfile).unwrap();
        decoder.read_to_end(&mut vf).unwrap();

        let mut fout2 = BufWriter::new(File::create(Path::new("asset/s3-2006-03-01.normal.json.gz.json")).unwrap());
        fout2.write(&vf).unwrap();
    }

}


// Gzip & Deflate
//   https://github.com/alexcrichton/flate2-rs
pub fn compress_decompress_deflate() {

    {
        // None, Fast, Best, Default
        let jsonfile = std::fs::File::open("asset/s3-2006-03-01.normal.json").unwrap();
        let mut reader = BufReader::new(jsonfile);
        let mut jsoncontent = Vec::new();
        reader.read_to_end(&mut jsoncontent).unwrap();

        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::Default);
        encoder.write(&jsoncontent).unwrap();
        let compressed = encoder.finish().unwrap();

        let mut fout = BufWriter::new(File::create(Path::new("asset/s3-2006-03-01.normal.json.deflate")).unwrap());
        fout.write(&compressed).unwrap();
    }

    {
        let mut vf = Vec::new();
        let jsonfile = std::fs::File::open("asset/s3-2006-03-01.normal.json.deflate").unwrap();

        let mut decoder =  DeflateDecoder::new(jsonfile);
        decoder.read_to_end(&mut vf).unwrap();

        let mut fout2 = BufWriter::new(File::create(Path::new("asset/s3-2006-03-01.normal.json.deflate.json")).unwrap());
        fout2.write(&vf).unwrap();
    }

}


// lz4
// https://github.com/bozaro/lz4-rs/
// http://blog.csdn.net/zhangskd/article/details/17009111
pub fn compress_decompress_lz4() {

    {
        // None, Fast, Best, Default
        let mut jsonfile = std::fs::File::open("asset/s3-2006-03-01.normal.json").unwrap();
        let mut reader = BufReader::new(jsonfile);
        let mut jsoncontent = Vec::new();
        reader.read_to_end(&mut jsoncontent).unwrap();

        let mut encoder = lz4::EncoderBuilder::new().build(Vec::new()).unwrap();
        encoder.write(&jsoncontent).unwrap();
        let (compressed, encoderesult) = encoder.finish();

        let mut fout = BufWriter::new(File::create(Path::new("asset/s3-2006-03-01.normal.json.lz4")).unwrap());
        fout.write(&compressed).unwrap();
    }

    {
        let mut vf = Vec::new();
        let jsonfile = std::fs::File::open("asset/s3-2006-03-01.normal.json.lz4").unwrap();

        let mut decoder =  lz4::Decoder::new(jsonfile).unwrap();
        decoder.read_to_end(&mut vf).unwrap();

        let mut fout2 = BufWriter::new(File::create(Path::new("asset/s3-2006-03-01.normal.json.lz4.json")).unwrap());
        fout2.write(&vf).unwrap();
    }

}


// snappy
//   https://github.com/JeffBelgum/rust-snappy
// waiting for pure rust version to be mature:
//   https://github.com/veddan/rust-snappy

// unable to install snapp c++ library in mac osx, so disable it
pub fn compress_decompress_snappy() {

    {
        // None, Fast, Best, Default
        let mut jsonfile = std::fs::File::open("asset/s3-2006-03-01.normal.json").unwrap();
        let mut reader = BufReader::new(jsonfile);
        let mut jsoncontent = Vec::new();
        reader.read_to_end(&mut jsoncontent).unwrap();


        let compressed = snappy::compress(&jsoncontent);

        let mut fout = BufWriter::new(File::create(Path::new("asset/s3-2006-03-01.normal.json.snappy")).unwrap());
        fout.write(&compressed).unwrap();
    }

    {
        let mut vf : Vec<u8> = Vec::new();
        let jsonfile = std::fs::File::open("asset/s3-2006-03-01.normal.json.lz4").unwrap();
        let mut reader = BufReader::new(jsonfile);
        let mut jsoncontent : Vec<u8> = Vec::new();
        reader.read_to_end(&mut jsoncontent).unwrap();

        let mut decompressed =  snappy::uncompress(&jsoncontent).unwrap();


        let mut fout2 = BufWriter::new(File::create(Path::new("asset/s3-2006-03-01.normal.json.snappy.json")).unwrap());
        fout2.write(&decompressed).unwrap();
    }
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use super::*;

    #[bench]
    fn bench_compress_decompress_gzip(bh: &mut Bencher) {
        bh.iter(|| {
            compress_decompress_gzip()
        });
    }

    #[bench]
    fn bench_compress_decompress_deflate(bh: &mut Bencher) {
        bh.iter(|| {
            compress_decompress_deflate()});
        }

    #[bench]
    fn bench_compress_decompress_lz4(bh: &mut Bencher) {
        bh.iter(|| {
            compress_decompress_lz4();});
        }
}

fn main() {
    decompress_brotli();
    compress_decompress_gzip();
    compress_decompress_deflate();
    compress_decompress_lz4();
}
