define_state_group!(data_states_group = {

    pub data_state <-- ( start_raw; ) {
        b'<' => ( emit_chars; start_raw; --> tag_open_state )
        eof  => ( emit_chars; emit_eof; )
        _    => ()
    }

    tag_open_state {
        b'!'  => ( --> markup_declaration_open_state )
        b'/'  => ( --> end_tag_open_state )
        alpha => ( create_start_tag; start_slice; --> tag_name_state )
        b'?'  => ( start_raw; --> bogus_comment_state )
        eof   => ( emit_chars; emit_eof; )
        _     => ( emit_chars; reconsume in data_state )
    }

    markup_declaration_open_state {
        eof => ( emit_eof; )
        _   => ( emit_eof; )
    }

    tag_name_state {
        eof => ( emit_eof; )
        _   => ( emit_eof; )
    }

    bogus_comment_state {
        b'>' => ( emit_comment; --> data_state )
        eof  => ( emit_comment; emit_eof; )
        _    => ()
    }

});
