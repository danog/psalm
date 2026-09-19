<?php

/**
 * The interface PHP's date classes share. Psalm's own stubs name it (DateTimeImmutable implements it)
 * without declaring it: in PHP it is reflected, and a compiled program has no reflection of it.
 */
interface DateTimeInterface
{
    // the format constants PHP's own DateTimeInterface carries: a compiled program has no constant
    // reflection, so they have to be declared to be seen
    public const ATOM = 'Y-m-d\\TH:i:sP';
    public const COOKIE = 'l, d-M-Y H:i:s T';
    public const ISO8601 = 'Y-m-d\\TH:i:sO';
    public const ISO8601_EXPANDED = 'X-m-d\\TH:i:sP';
    public const RFC822 = 'D, d M y H:i:s O';
    public const RFC850 = 'l, d-M-y H:i:s T';
    public const RFC1036 = 'D, d M y H:i:s O';
    public const RFC1123 = 'D, d M Y H:i:s O';
    public const RFC7231 = 'D, d M Y H:i:s \\G\\M\\T';
    public const RFC2822 = 'D, d M Y H:i:s O';
    public const RFC3339 = 'Y-m-d\\TH:i:sP';
    public const RFC3339_EXTENDED = 'Y-m-d\\TH:i:s.vP';
    public const RSS = 'D, d M Y H:i:s O';
    public const W3C = 'Y-m-d\\TH:i:sP';

    // without native return types, as Psalm's own stub of DateTimeImmutable declares them: a native
    // one here would be a signature mismatch against that stub
    /** @return string */
    public function format(string $format);

    /** @return int */
    public function getTimestamp();

    /** @return int */
    public function getOffset();

    /** @return DateInterval */
    public function diff(DateTimeInterface $targetObject, bool $absolute = false);
}

/** A timezone; Psalm's own stub describes it, and a compiled program needs the class to exist. */
class DateTimeZone
{
    public function __construct(private string $timezone = 'UTC')
    {
    }

    public function getName(): string
    {
        return $this->timezone;
    }
}

/** The difference between two dates; a compiled analyzer only needs its shape. */
class DateInterval
{
    public int $y = 0;
    public int $m = 0;
    public int $d = 0;
    public int $h = 0;
    public int $i = 0;
    public int $s = 0;
    public float $f = 0.0;
    public int $invert = 0;

    /** @var int|false */
    public int|false $days = false;

    public function __construct(string $duration = 'P0D')
    {
    }

    public function format(string $format): string
    {
        return $format;
    }
}

/**
 * A minimal DateTime for the compiled analyzer: it only has to tell whether a modifier string is one PHP's
 * date parser accepts (DateTimeModifyReturnTypeProvider infers `DateTime|false` from that). Absolute dates are
 * not modelled; the recognised relative formats are the common ones ("+1 day", "next monday", "midnight", ...).
 */
class DateTime implements DateTimeInterface
{
    public function format(string $format): string
    {
        return $format;
    }

    public function getTimestamp(): int
    {
        return 0;
    }

    public function getOffset(): int
    {
        return 0;
    }

    public function diff(DateTimeInterface $targetObject, bool $absolute = false): DateInterval
    {
        return new DateInterval();
    }

    public function getTimezone(): DateTimeZone
    {
        return new DateTimeZone();
    }

    public function setTimezone(DateTimeZone $timezone): DateTime
    {
        return $this;
    }

    public function add(DateInterval $interval): DateTime
    {
        return $this;
    }

    public function sub(DateInterval $interval): DateTime
    {
        return $this;
    }

    public function setTimestamp(int $timestamp): DateTime
    {
        return $this;
    }

    public function setDate(int $year, int $month, int $day): DateTime
    {
        return $this;
    }

    public function setTime(int $hour, int $minute, int $second = 0, int $microsecond = 0): DateTime
    {
        return $this;
    }

    public static function createFromInterface(DateTimeInterface $object): DateTime
    {
        return new DateTime();
    }

    public function __construct(string $datetime = 'now', ?DateTimeZone $timezone = null)
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
