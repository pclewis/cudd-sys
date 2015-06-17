$re_sym = qr/[a-zA-Z_][a-zA-Z0-9_]*/;
$re_type = qr/(?:(?:const|unsigned|short|long) )*$re_sym(?:\s*(?:\*|const\s*\*\s*)+\s*|\s+)/;

%type_map = (
    'DdNode' => 'DdNode',
    'DdManager' => 'DdManager',
    'int' => 'c_int',
    'unsignedint' => 'c_uint',
    'long' => 'c_long',
    'unsignedlong' => 'c_ulong',
    'char' => 'c_char',
    'constchar' => 'c_char',
    'FILE' => 'FILE',
);

sub t {
    my $t = shift(@_); my $tt = $t; my $ta = $t;
    $tt =~ s/\s+//g; $tt =~ s/\*+//g; # no stars or spaces
    $ta =~ s/(const\s*)?\*+//g; $ta =~ s/\s+$//; # no stars or trailing spaces
    my $m = ($t =~ /^CUDD_/i) ? $t : ($type_map{$tt} || "c_$ta");
    while ($t =~ /(const\s*)?\*/g) {
        if($1 =~ /const/) {
            $m = '*const ' . $m;
        } else {
            $m = '*mut ' . $m;
        }
    }
    return $m;
}


while(<STDIN>) {

    if($_ =~ /^extern ($re_type)($re_sym)\s*\(\s*((?:($re_type)($re_sym)\s*,?\s*)*)\)/) {
        #(Cudd_\S+)\s*\((($re_type)($re_sym),?\s*)*\)/) {
        my $type = t($1), $args = $3, @al = ();
        print "fn $2(";
        while( $args =~ /($re_type)($re_sym)/g ) {
            push(@al, $2 . ': ' . t($1));
        }
        print join(", ", @al);
        print ") -> $type;\n";
    } elsif($_ =~ /^extern ($re_type)($re_sym)\s*\(($re_type)/) {
        print "M1: $_\n";
    } elsif (/^extern/) {
        print "No match: $_\n";
    }
}
