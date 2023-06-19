use crate::openai::OpenAI;

use std::io::Cursor;

use mp3lame_encoder::{
    Builder,
    Id3Tag,
    MonoPcm,
    FlushNoGap
};



pub struct VoiceTranscription;

impl VoiceTranscription{
    pub async fn run(voice:Vec<u8>)->String{
        let buffer={
            let (samples,_)=ogg_opus::decode::<Cursor<Vec<u8>>,48000>(Cursor::new(voice)).unwrap();

            let mut mp3_encoder=Builder::new().expect("Create LAME builder");
            mp3_encoder.set_num_channels(1).expect("set channels");
            mp3_encoder.set_sample_rate(44_100).expect("set sample rate");
            mp3_encoder.set_brate(mp3lame_encoder::Bitrate::Kbps192).expect("set brate");
            mp3_encoder.set_quality(mp3lame_encoder::Quality::Best).expect("set quality");
            mp3_encoder.set_id3_tag(Id3Tag{
                title:b"My title",
                artist:&[],
                album:b"My album",
                year:b"Current year",
                comment:b"Just my comment",
            });
            let mut mp3_encoder=mp3_encoder.build().expect("To initialize LAME encoder");

            let input=MonoPcm(&samples);

            let mut mp3_out_buffer=Vec::new();
            mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(samples.len()));
            let encoded_size=mp3_encoder.encode(input,mp3_out_buffer.spare_capacity_mut()).expect("To encode");
            unsafe{
                mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
            }

            let encoded_size=mp3_encoder.flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut()).expect("to flush");
            unsafe{
                mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
            }

            mp3_out_buffer
        };

        OpenAI::send_voice(buffer).await
    }
}