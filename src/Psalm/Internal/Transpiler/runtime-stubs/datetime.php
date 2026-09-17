<?php

/**
 * A minimal DateTime for the compiled analyzer: it only has to tell whether a modifier string is one PHP's
 * date parser accepts (DateTimeModifyReturnTypeProvider infers `DateTime|false` from that). Absolute dates are
 * not modelled; the recognised relative formats are the common ones ("+1 day", "next monday", "midnight", ...).
 */
class DateTime
{
    public function __construct(string $datetime = 'now')
    {
    }

    /**
     * @return DateTime|false
     */
    public function modify(string $modifier): DateTime|false
    {
        $m = strtolower(trim($modifier));
        if ($m === '') {
            return false;
        }
        $unit = '(sec|second|min|minute|hour|day|week|fortnight|month|year|weekday|msec|millisecond|usec|microsecond)s?';
        $weekday = '(mon|tue|wed|thu|fri|sat|sun)[a-z]*';
        $keywords = ['now', 'today', 'tomorrow', 'yesterday', 'midnight', 'noon', 'first day of this month', 'last day of this month',
            'first day of next month', 'last day of next month', 'first day of last month', 'last day of last month'];
        if (in_array($m, $keywords, true)) {
            return $this;
        }
        $parts = preg_split('/\s*,\s*|\s+(?=[+-]?\d)/', $m) ?: [];
        foreach ($parts as $part) {
            if ($part === '') {
                continue;
            }
            if (preg_match('/^[+-]?\d+\s*' . $unit . '(\s+ago)?$/', $part) === 1
                || preg_match('/^(next|last|this|previous)\s+(' . $unit . '|' . $weekday . ')$/', $part) === 1
                || preg_match('/^' . $weekday . '(\s+(next|last|this)\s+week)?$/', $part) === 1
                || preg_match('/^(first|last)\s+day\s+of(\s+(next|last|this|previous)\s+(month|year))?$/', $part) === 1
                || preg_match('/^(first|last|next|previous|second|third|fourth|fifth)\s+' . $weekday . '\s+of(\s+.*)?$/', $part) === 1
                || preg_match('/^\d{1,2}:\d{2}(:\d{2})?$/', $part) === 1
                || preg_match('/^\d{4}-\d{2}-\d{2}$/', $part) === 1
            ) {
                continue;
            }
            return false;
        }
        return $this;
    }

    public function format(string $format): string
    {
        return '';
    }
}
